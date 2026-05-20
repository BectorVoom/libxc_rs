//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3375/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3375<F: Float>(t41330: F, t41332: F, t52047: F, t52049: F, t52051: F, t63399: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63462: F, t63464: F) -> F {
    let t63466 = F::new(8.0) / F::new(27.0) * t52047 + F::new(4.0) / F::new(27.0) * t52049 + F::new(20.0) / F::new(81.0) * t52051 - F::new(8.0) * t63399 - F::new(4.0) / F::new(27.0) * t41330 - F::new(8.0) / F::new(81.0) * t41332 + F::new(2.0) / F::new(9.0) * t63447 - t63451 / F::new(3.0) - F::new(8.0) / F::new(81.0) * t63453 - F::new(4.0) / F::new(9.0) * t63457 + F::new(8.0) / F::new(27.0) * t63459 + F::new(4.0) / F::new(3.0) * t63462 - F::new(4.0) / F::new(27.0) * t63464;
    t63466
}
