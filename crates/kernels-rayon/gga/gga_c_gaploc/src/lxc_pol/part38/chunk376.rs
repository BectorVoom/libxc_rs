//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 376/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk376(t3177: f64, t912: f64, t587: f64, t1201: f64, t124: f64, t60: f64, t1390: f64, t40: f64, t203: f64, t574: f64, t2488: f64, t2487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3178 = t912 * t3177;
    let t3179 = t587 * t3178;
    let t3180 = 0.38342925953920749676e0_f64 * t3179;
    let t3190 = t60 * t1201 * t124;
    let t3191 = t1390 * t40;
    let t3192 = t3191 * t203;
    let t3193 = t3190 * t3192;
    let t3194 = t574 * t3193;
    let t3196 = t2488 * t3177;
    let t3197 = t2487 * t3196;
    (t3178, t3180, t3190, t3191, t3192, t3193, t3194, t3196, t3197)
}
