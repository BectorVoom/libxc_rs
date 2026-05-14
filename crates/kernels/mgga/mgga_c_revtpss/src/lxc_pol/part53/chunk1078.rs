//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1078/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1078<F: Float>(t125355: F, t125357: F, t125359: F, t125363: F, t125366: F, t125368: F, t125370: F, t125372: F, t125374: F, t125377: F, t125379: F, t125381: F, t125383: F, t125385: F, t125387: F, t125389: F, t125391: F, t129246: F, t32178: F) -> (F,) {
    let t129502 = t32178 + 2.0 * t125355 + 2.0 * t125357 + 2.0 * t125359 + 2.0 * t125363 + 2.0 * t125366 + 2.0 * t125368 + 2.0 * t125370 + 2.0 * t125372 + 2.0 * t125374 + t125377 + t125379 + t125381 + t125383 + t125385 + t125387 + t125389 + t125391 + t129246;
    (t129502,)
}
