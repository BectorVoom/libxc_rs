//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3367/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3367<F: Float>(t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F) -> F {
    let t63440 = -F::new(4.0) / F::new(9.0) * t63359 + F::new(4.0) / F::new(3.0) * t63361 + F::new(4.0) / F::new(3.0) * t63366 - F::new(2.0) * t63369 - F::new(8.0) / F::new(9.0) * t63371 - F::new(2.0) * t63374 + F::new(4.0) / F::new(3.0) * t52033 + F::new(32.0) / F::new(27.0) * t52035 - F::new(32.0) / F::new(81.0) * t52037 - F::new(8.0) / F::new(9.0) * t52039 - F::new(4.0) / F::new(9.0) * t52041 - F::new(8.0) / F::new(9.0) * t52045;
    t63440
}
