//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1089/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1089<F: Float>(t11991: F, t33476: F, t11994: F, t11320: F, t11938: F, t928: F, t33460: F, t33462: F, t33464: F, t33466: F, t33468: F, t33470: F, t33472: F, t33474: F) -> F {
    let t33477 = t33476 * t11991;
    let t33479 = t33476 * t11994;
    let t33482 = t928 * t11320 * t11938;
    let t33484 = -F::new(0.16882049790461501058e-6) * t33460 - F::new(0.22509399720615334744e-6) * t33462 - F::new(0.90579542097823505428e-7) * t33464 + F::new(0.35170937063461460536e-8) * t33466 - F::new(0.77294542590142724635e-6) * t33468 + F::new(0.1374296967252737644e-5) * t33470 + F::new(0.11254699860307667372e-6) * t33472 - F::new(0.2845640240200497334e-7) * t33474 - F::new(0.4637672555408563478e-4) * t33477 + F::new(0.4637672555408563478e-4) * t33479 + F::new(0.38647271295071362318e-6) * t33482;
    t33484
}
