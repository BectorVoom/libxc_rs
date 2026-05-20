//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2216/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2216<F: Float>(t104181: F, t104185: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t29364: F, t29367: F, t29412: F, t29538: F, t29554: F, t7576: F, t7579: F, t7706: F, t7709: F, t8144: F) -> F {
    let t111493 = t29554 * t7579 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t28112 * t8144 + F::new(2.0) / F::new(3.0) * t28116 * t8144 + F::new(2.0) / F::new(3.0) * t28119 * t8144 + F::new(2.0) / F::new(3.0) * t7709 * t29364 + F::new(2.0) / F::new(3.0) * t7709 * t29367 + F::new(2.0) / F::new(3.0) * t29538 * t7576 + F::new(2.0) / F::new(3.0) * t29538 * t7579 + F::new(5.0) / F::new(3.0) * t104181 * t7706 + F::new(5.0) / F::new(3.0) * t104185 * t7706 + F::new(5.0) / F::new(3.0) * t29412 * t28105 + F::new(5.0) / F::new(3.0) * t29412 * t28109;
    t111493
}
