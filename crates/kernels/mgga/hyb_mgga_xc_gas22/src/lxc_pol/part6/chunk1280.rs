//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1280/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1280<F: Float>(t20022: F, t20070: F, t20073: F, t23100: F, t23102: F, t23105: F, t23108: F, t23111: F, t23113: F, t23116: F, t23118: F, t23120: F, t23124: F, t23128: F, t23139: F, t23572: F, t23575: F, t7831: F, t9858: F) -> F {
    let t27706 = -t23100 / F::new(16.0) - t23102 / F::new(16.0) - t23105 / F::new(32.0) - t23108 / F::new(32.0) - t23111 / F::new(48.0) + t23113 / F::new(24.0) - F::new(5.0) / F::new(144.0) * t23116 - t23118 / F::new(16.0) - t23120 / F::new(32.0) - F::new(5.0) / F::new(432.0) * t20022 - F::new(3.0) / F::new(16.0) * t7831 * t9858 - F::new(5.0) / F::new(144.0) * t23124 - F::new(41.0) / F::new(48.0) * t23128 - t23139 / F::new(36.0) + t20070 / F::new(144.0) + t20073 / F::new(144.0) + t23572 / F::new(24.0) + t23575 / F::new(24.0);
    t27706
}
