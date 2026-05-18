//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1296/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1296<F: Float>(t10437: F, t136: F, t550: F, t24001: F, t24003: F, t24006: F, t24011: F, t24013: F, t28086: F, t28089: F, t28092: F, t28095: F, t28097: F, t28102: F, t28104: F, t28106: F, t28108: F, t28111: F, t28115: F, t28119: F, t28121: F, t675: F, t684: F, t687: F) -> F {
    let t28125 = t136 * t550 * t10437;
    let t28128 = -t28086 / F::new(48.0) - F::new(7.0) / F::new(48.0) * t28089 - t28092 / F::new(48.0) - F::new(7.0) / F::new(48.0) * t28095 - t684 * t687 * t28097 * t675 / F::new(32.0) - F::new(7.0) / F::new(16.0) * t28102 - t28104 / F::new(16.0) - F::new(7.0) / F::new(16.0) * t28106 - t28108 / F::new(32.0) - t28111 / F::new(32.0) - F::new(5.0) / F::new(144.0) * t24001 + t28115 / F::new(96.0) - t24003 / F::new(16.0) + t24006 / F::new(24.0) + t28119 / F::new(48.0) + t28121 / F::new(48.0) + F::new(41.0) / F::new(48.0) * t24011 - t28125 / F::new(16.0) + t24013 / F::new(8.0);
    t28128
}
