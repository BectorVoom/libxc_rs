//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3815/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3815<F: Float>(t198: F, t3828: F, t39483: F, t39520: F, t39528: F, t73345: F, t73350: F, t73353: F, t73354: F, t73355: F, t73356: F, t73357: F, t73358: F) -> F {
    let t73359 = F::cast_from(12.0_f64) * t198 * t3828 * t73345 - t39483 + t39520 - t39528 + t73350 - t73353 - t73354 + t73355 + t73356 + t73357 - t73358;
    t73359
}
