//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 779/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk779(t13212: f64, t2508: f64, t12613: f64, t12624: f64, t13157: f64, t7226: f64, t12630: f64, t1024: f64, t3270: f64, t10677: f64, t883: f64, t2562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13214 = 0.23071578690426672851e-1_f64 * t2508 * t13212;
    let t13215 = 0.64087718584518535698e-3_f64 * t12613;
    let t13216 = 0.64087718584518535698e-3_f64 * t12624;
    let t13217 = t7226 * t13157;
    let t13219 = 0.46143157380853345701e-1_f64 * t2508 * t13217;
    let t13220 = 0.64087718584518535698e-3_f64 * t12630;
    let t13221 = t3270 * t1024;
    let t13223 = 0.76905262301422242837e-2_f64 * t2508 * t13221;
    let t13224 = t883 * t10677;
    let t13225 = t2562 * t13224;
    (t13214, t13215, t13216, t13217, t13219, t13220, t13221, t13223, t13225)
}
