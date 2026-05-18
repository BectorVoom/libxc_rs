//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1175/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1175<F: Float>(t125377: F, t125379: F, t125381: F, t125383: F, t125385: F, t125387: F, t125389: F, t125391: F, t129489: F, t129490: F, t131321: F, t32176: F, t32178: F, t8564: F) -> F {
    let t131387 = F::new(4.0) * t129489 + F::new(4.0) * t129490 + t8564 + t32176 + t32178 + t131321 + t125377 + t125379 + t125381 + t125383 + t125385 + t125387 + t125389 + t125391;
    t131387
}
