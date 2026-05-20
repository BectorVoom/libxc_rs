//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3606/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3606<F: Float>(t56176: F, t56183: F, t56185: F, t56187: F, t56189: F, t56209: F, t56212: F, t56214: F, t56216: F, t56228: F, t68363: F, t68366: F) -> F {
    let t68443 = -F::new(16.0) / F::new(3.0) * t68363 + F::new(40.0) / F::new(27.0) * t68366 - F::new(32.0) / F::new(81.0) * t56176 + F::new(32.0) / F::new(27.0) * t56183 - F::new(8.0) / F::new(9.0) * t56185 - F::new(4.0) / F::new(9.0) * t56187 - F::new(4.0) / F::new(3.0) * t56189 + F::new(8.0) / F::new(27.0) * t56209 + F::new(4.0) / F::new(27.0) * t56212 + F::new(8.0) / F::new(9.0) * t56214 - F::new(20.0) / F::new(81.0) * t56216 + F::new(16.0) / F::new(27.0) * t56228;
    t68443
}
