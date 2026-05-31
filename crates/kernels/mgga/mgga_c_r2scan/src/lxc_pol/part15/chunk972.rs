//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 972/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk972<F: Float>(t322: F, t11059: F, t3370: F, t833: F, t1074: F, t1299: F, t1295: F, t829: F, t1292: F, t1300: F, t327: F, t3373: F, t6693: F, t834: F) -> (F, F, F, F) {
    let t324 = F::cast_from(0.0_f64) < t322;
    let t11060 = piecewise3::<F>(t324, F::cast_from(0.0_f64), t11059);
    let t11063 = t3370 * t833;
    let t11066 = t1074 * t1299;
    let t11071 = t1074 * t1295;
    let t11074 = t3370 * t829;
    let t11077 = t1074 * t1292;
    let t11082 = -F::cast_from(0.64e0_f64) * t11060 * t327 - F::cast_from(0.256e1_f64) * t11063 * t829 - F::cast_from(0.384e1_f64) * t11066 * t1295 - F::cast_from(0.128e1_f64) * t3373 * t1292 - F::cast_from(0.384e1_f64) * t6693 * t11071 - F::cast_from(0.256e1_f64) * t1300 * t11074 - F::cast_from(0.128e1_f64) * t1300 * t11077 - F::cast_from(0.64e0_f64) * t834 * t11060;
    (t11060, t11063, t11066, t11082)
}
