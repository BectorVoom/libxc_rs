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
    let t28128 = -t28086 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t28089 - t28092 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t28095 - t684 * t687 * t28097 * t675 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t28102 - t28104 / F::cast_from(16.0_f64) - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t28106 - t28108 / F::cast_from(32.0_f64) - t28111 / F::cast_from(32.0_f64) - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t24001 + t28115 / F::cast_from(96.0_f64) - t24003 / F::cast_from(16.0_f64) + t24006 / F::cast_from(24.0_f64) + t28119 / F::cast_from(48.0_f64) + t28121 / F::cast_from(48.0_f64) + F::cast_from(41.0_f64) / F::cast_from(48.0_f64) * t24011 - t28125 / F::cast_from(16.0_f64) + t24013 / F::cast_from(8.0_f64);
    t28128
}
