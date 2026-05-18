//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 734/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk734<F: Float>(t2911: F, t8470: F, t2899: F, t426: F, t425: F, t462: F, t2886: F, t458: F, t8465: F, t2879: F, t8444: F, t8446: F, t8455: F, t8457: F, t8461: F, t8463: F, t8467: F) -> (F, F, F, F, F) {
    let t8471 = t2911 * t8470;
    let t8473 = t426 * t2899;
    let t8475 = t462 * t425;
    let t8476 = t8475 * t2886;
    let t8478 = t8465 * t458;
    let t8479 = t2879 * t8478;
    let t8481 = -F::new(0.12360406057797588768e-3) * t8444 + F::new(0.70184646495910842219e-3) * t8446 + F::new(0.43056987158198508472e-6) * t8455 - F::new(0.772525378612349298e-5) * t8457 - F::new(0.43449121406768801912e-5) * t8461 + F::new(0.5407677650286445086e-4) * t8463 - F::new(0.20855578275249024918e-2) * t8467 + F::new(0.60736713313768998074e-4) * t8471 - F::new(0.20855578275249024918e-2) * t8473 - F::new(0.6951859425083008306e-4) * t8476 - F::new(0.20855578275249024918e-2) * t8479;
    (t8471, t8473, t8476, t8479, t8481)
}
