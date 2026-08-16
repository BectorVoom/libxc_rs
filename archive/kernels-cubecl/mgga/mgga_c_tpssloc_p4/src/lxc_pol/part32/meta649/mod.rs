//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta649 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2073;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2074;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta649<F: Float>(t562: F, t91005: F, t22751: F, t26385: F, t26389: F, t26467: F, t6914: F, t26426: F, t81046: F, t22690: F, t7732: F, t81195: F, t22832: F, t5234: F, t1336: F, t22759: F, t5252: F, t836: F, t5293: F, t80820: F, t1831: F, t80869: F, t22783: F, t5314: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91006, t91011, t91065, t91077, t91078, t91081) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2073::<F>(t562, t91005, t22751, t26385, t26389, t26467, t6914, t26426, t81046, t22690, t7732, t81195);
        let (t91100, t91114, t91121, t91136, t91137) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2074::<F>(t22832, t5234, t1336, t22759, t5252, t836, t5293, t80820, t1831, t80869, t22783, t5314);
    (t91006, t91011, t91065, t91077, t91078, t91081, t91100, t91114, t91121, t91136, t91137)
}
