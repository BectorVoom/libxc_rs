//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 900/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk900<F: Float>(t322: F, t9675: F, t2941: F, t833: F, t1299: F, t2944: F, t829: F, t1013: F, t2394: F, t1300: F, t2397: F, t327: F, t6693: F, t834: F) -> (F, F, F, F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t9676 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t9675);
    let t9679 = t2941 * t833;
    let t9684 = t2944 * t1299;
    let t9687 = t2944 * t829;
    let t9690 = t1013 * t2394;
    let t9693 = t2941 * t829;
    let t9698 = -F::cast_from(0.64e0_f64) * t9676 * t327 - F::cast_from(0.128e1_f64) * t9679 * t829 - F::cast_from(0.256e1_f64) * t2397 * t2394 - F::cast_from(0.384e1_f64) * t9684 * t829 - F::cast_from(0.384e1_f64) * t6693 * t9687 - F::cast_from(0.256e1_f64) * t1300 * t9690 - F::cast_from(0.128e1_f64) * t1300 * t9693 - F::cast_from(0.64e0_f64) * t834 * t9676;
    (t9676, t9687, t9690, t9693, t9698)
}
