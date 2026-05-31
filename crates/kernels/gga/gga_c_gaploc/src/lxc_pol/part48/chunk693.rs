//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 693/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk693<F: Float>(t3553: F, t921: F, t4349: F, t1016: F, t3418: F, t1382: F, t2355: F, t3599: F, t11402: F, t895: F, t11386: F, t2778: F, t3338: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13343 = t3553 * t921;
    let t13345 = F::cast_from(6.0_f64) * t4349 * t13343;
    let t13346 = t1016 * t3418;
    let t13348 = F::cast_from(4.0_f64) * t1382 * t13346;
    let t13349 = t2355 * t3599;
    let t13350 = t3599 * t921;
    let t13352 = F::cast_from(2.0_f64) * t1382 * t13350;
    let t13354 = F::cast_from(0.35750489951850426669e0_f64) * t895 * t11402;
    let t13356 = F::cast_from(0.35750489951850426669e0_f64) * t895 * t11386;
    let t13359 = t2778 * t3338;
    (t13343, t13345, t13346, t13348, t13349, t13350, t13352, t13354, t13356, t13359)
}
