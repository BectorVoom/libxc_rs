//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1685;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta440<F: Float>(t107: F, t240: F, t625: F, t656: F, t666: F, t2331: F, t63: F, t43: F, t614: F, t2267: F, t38: F, t33: F, t6504: F, t2240: F, t6489: F, t9239: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22469, t22470, t22471, t22473, t22502, t22505, t22510, t22522) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1685::<F>(t107, t240, t625, t656, t666, t2331, t63, t43, t614, t2267, t38, t33, t6504);
        let (t22523, t22544) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1686::<F>(t2240, t22522, t6489, t9239);
    (t22469, t22470, t22471, t22473, t22502, t22505, t22510, t22522, t22523, t22544)
}
