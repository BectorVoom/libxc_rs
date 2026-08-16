//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 387/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk387(t203: f64, t3191: f64, t3190: f64, t574: f64, t2488: f64, t3177: f64, t2487: f64, t1565: f64, t3085: f64, t568: f64, t3116: f64, t600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3192 = t3191 * t203;
    let t3193 = t3190 * t3192;
    let t3194 = t574 * t3193;
    let t3196 = t2488 * t3177;
    let t3197 = t2487 * t3196;
    let t3198 = 0.38342925953920749676e0_f64 * t3197;
    let t3199 = t1565 * t3085;
    let t3200 = t568 * t3199;
    let t3203 = t600 * t3116;
    (t3192, t3193, t3194, t3196, t3197, t3198, t3199, t3200, t3203)
}
