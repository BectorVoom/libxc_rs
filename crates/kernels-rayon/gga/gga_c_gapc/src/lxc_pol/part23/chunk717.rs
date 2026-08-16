//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 717/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk717(t2911: f64, t8470: f64, t2899: f64, t426: f64, t425: f64, t462: f64, t2886: f64, t458: f64, t8465: f64, t2879: f64, t8444: f64, t8446: f64, t8455: f64, t8457: f64, t8461: f64, t8463: f64, t8467: f64) -> f64 {
    let t8471 = t2911 * t8470;
    let t8473 = t426 * t2899;
    let t8475 = t462 * t425;
    let t8476 = t8475 * t2886;
    let t8478 = t8465 * t458;
    let t8479 = t2879 * t8478;
    let t8481 = -0.12360406057797588768e-3_f64 * t8444 + 0.70184646495910842219e-3_f64 * t8446 + 0.43056987158198508472e-6_f64 * t8455 - 0.772525378612349298e-5_f64 * t8457 - 0.43449121406768801912e-5_f64 * t8461 + 0.5407677650286445086e-4_f64 * t8463 - 0.20855578275249024918e-2_f64 * t8467 + 0.60736713313768998074e-4_f64 * t8471 - 0.20855578275249024918e-2_f64 * t8473 - 0.6951859425083008306e-4_f64 * t8476 - 0.20855578275249024918e-2_f64 * t8479;
    t8481
}
