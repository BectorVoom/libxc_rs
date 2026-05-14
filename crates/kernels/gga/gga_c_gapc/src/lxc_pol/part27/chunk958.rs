//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 958/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk958<F: Float>(t30167: F, t33202: F, t11997: F, t2778: F, t11937: F, t11781: F, t9999: F, t16182: F, t29033: F, t11483: F, t928: F, t11991: F, t11994: F, t11320: F, t11938: F, t33460: F, t33462: F, t33464: F) -> (F,) {
    let t33466 = t33202 * t30167;
    let t33468 = t11997 * t2778;
    let t33470 = t11937 * t2778;
    let t33472 = t11781 * t9999;
    let t33474 = t29033 * t16182;
    let t33476 = t928 * t11483;
    let t33477 = t33476 * t11991;
    let t33479 = t33476 * t11994;
    let t33482 = t928 * t11320 * t11938;
    let t33484 = -0.16882049790461501058e-6 * t33460 - 0.22509399720615334744e-6 * t33462 - 0.90579542097823505428e-7 * t33464 + 0.35170937063461460536e-8 * t33466 - 0.77294542590142724635e-6 * t33468 + 0.1374296967252737644e-5 * t33470 + 0.11254699860307667372e-6 * t33472 - 0.2845640240200497334e-7 * t33474 - 0.4637672555408563478e-4 * t33477 + 0.4637672555408563478e-4 * t33479 + 0.38647271295071362318e-6 * t33482;
    (t33484,)
}
