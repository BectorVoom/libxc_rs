//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3807/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3807<F: Float>(t18123: F, t20692: F, t3794: F, t5023: F, t5505: F, t68942: F, t68946: F, t68949: F, t68951: F, t68954: F, t68956: F, t68959: F, t68961: F, t68963: F, t68965: F, t68967: F, t68969: F) -> F {
    let t73283 = -F::new(2.0) * t18123 * t5023 * t5505 - t20692 * t3794 * t5023 + t68942 + t68946 + t68949 + t68951 + t68954 + t68956 - t68959 - t68961 - t68963 + t68965 + t68967 - t68969;
    t73283
}
