//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 905/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk905<F: Float>(t322: F, t2441: F, t352: F, t2983: F, t6755: F, t1338: F, t2987: F, t9675: F, t1348: F, t6767: F, t1019: F, t2405: F, t2437: F, t2438: F, t2445: F, t2951: F, t2953: F, t2991: F, t330: F, t837: F, t855: F, t9698: F, t9731: F, t9756: F) -> (F, F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t9760 = t352 * t2441;
    let t9763 = t6755 * t2983;
    let t9766 = t1338 * t2987;
    let t9769 = piecewise3::<F>(t332, t9675, F::cast_from(0.0_f64));
    let t9773 = t1348 * t2987;
    let t9778 = t6767 * t2983;
    let t9782 = piecewise5::<F>(t323, t2951 * t837 * t330 + t2953 * t837 * t330 + F::cast_from(2.0_f64) * t1019 * t2405 + t9698 * t330, t331, t9731 + t9756, -F::cast_from(0.63e1_f64) * t2991 * t2438 - F::cast_from(0.42e1_f64) * t2437 * t9760 - F::cast_from(0.945e1_f64) * t9763 * t2438 - F::cast_from(0.21e1_f64) * t9766 * t2438 - F::cast_from(0.105e1_f64) * t855 * t9769 * t352 - F::cast_from(0.1575e1_f64) * t9773 * t2438 - F::cast_from(0.315e1_f64) * t2445 * t9760 - F::cast_from(0.23625e1_f64) * t9778 * t2438);
    (t9760, t9769, t9782)
}
