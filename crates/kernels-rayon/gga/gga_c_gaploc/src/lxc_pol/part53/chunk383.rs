//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 383/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk383(t3177: f64, t912: f64, t587: f64, t3085: f64, t600: f64, t568: f64, t3116: f64, t569: f64, t1201: f64, t124: f64, t60: f64, t1390: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3178 = t912 * t3177;
    let t3179 = t587 * t3178;
    let t3180 = 0.38342925953920749676e0_f64 * t3179;
    let t3181 = t600 * t3085;
    let t3182 = t568 * t3181;
    let t3185 = t569 * t3116;
    let t3186 = t568 * t3185;
    let t3190 = t60 * t1201 * t124;
    let t3191 = t1390 * t40;
    (t3178, t3179, t3180, t3181, t3182, t3185, t3186, t3190, t3191)
}
