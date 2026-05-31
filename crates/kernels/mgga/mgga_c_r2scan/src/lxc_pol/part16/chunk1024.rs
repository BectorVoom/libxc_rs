//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1024/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1024<F: Float>(t322: F, t2983: F, t352: F, t12601: F, t10533: F, t11148: F, t11162: F, t12002: F, t12009: F, t12622: F, t12624: F, t12627: F, t12629: F, t12656: F, t12681: F, t330: F, t3413: F, t3420: F, t3675: F, t855: F) -> (F, F, F) {
    let t323 = t322 <= F::cast_from(0.0_f64);
    let t331 = t322 <= F::cast_from(0.25e1_f64);
    let t332 = F::cast_from(0.25e1_f64) < t322;
    let t12683 = t352 * t2983;
    let t12692 = piecewise3::<F>(t332, t12601, F::cast_from(0.0_f64));
    let t12703 = piecewise5::<F>(t323, t12622 * t330 + F::cast_from(2.0_f64) * t12624 * t330 + t12627 * t330 + t12629 * t330, t331, t12656 + t12681, -F::cast_from(0.63e1_f64) * t3420 * t12683 - F::cast_from(0.42e1_f64) * t12002 * t3675 - F::cast_from(0.945e1_f64) * t11148 * t12683 - F::cast_from(0.21e1_f64) * t3413 * t10533 - F::cast_from(0.105e1_f64) * t855 * t12692 * t352 - F::cast_from(0.315e1_f64) * t12009 * t3675 - F::cast_from(0.1575e1_f64) * t3420 * t10533 - F::cast_from(0.23625e1_f64) * t11162 * t12683);
    (t12683, t12692, t12703)
}
