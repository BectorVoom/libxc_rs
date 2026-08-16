//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 936/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk936(t30194: f64, t30197: f64, t30199: f64, t30229: f64, t30232: f64, t30238: f64, t30242: f64, t30246: f64, t30339: f64, t30396: f64, t30405: f64, t30421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32385 = 0.31448092289604152069e-3_f64 * t30194;
    let t32386 = 0.62896184579208304137e-3_f64 * t30197;
    let t32387 = 0.25724410870841842183e-2_f64 * t30199;
    let t32397 = 0.56606566121287473723e-2_f64 * t30229;
    let t32398 = 0.83861579438944405516e-2_f64 * t30232;
    let t32401 = 0.21437009059034868486e-3_f64 * t30238;
    let t32403 = 0.42874018118069736972e-3_f64 * t30242;
    let t32404 = 0.68026775414003982662e-1_f64 * t30246;
    let t32435 = 0.12862205435420921092e-2_f64 * t30339;
    let t32456 = 5.0_f64 / 64.0_f64 * t30396;
    let t32458 = 0.25724410870841842183e-2_f64 * t30405;
    let t32461 = 0.37737710747524982482e-2_f64 * t30421;
    (t32385, t32386, t32387, t32397, t32398, t32401, t32403, t32404, t32435, t32456, t32458, t32461)
}
