//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 709/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk709(t13525: f64, t169: f64, t299: f64, t706: f64, t1035: f64, t3433: f64, t13177: f64, t13488: f64, t13490: f64, t13494: f64, t13497: f64, t13498: f64, t13501: f64, t13504: f64, t13509: f64, t2508: f64, t270: f64) -> (f64, f64, f64, f64) {
    let t13527 = t13525 * t169 * t299;
    let t13528 = t706 * t13527;
    let t13531 = t1035 * t3433;
    let t13534 = 0.1281754371690370714e-2_f64 * t13177 - t13488 - 0.96131577876777803546e-3_f64 * t13490 + t13494 + t13497 - 0.46143157380853345702e-1_f64 * t2508 * t13498 + t13501 + 0.64087718584518535696e-3_f64 * t13504 - t13509 + 0.76905262301422242837e-2_f64 * t270 * t13528 + 0.15381052460284448567e-1_f64 * t2508 * t13531;
    (t13527, t13528, t13531, t13534)
}
