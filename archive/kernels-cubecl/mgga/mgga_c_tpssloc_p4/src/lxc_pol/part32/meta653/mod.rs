//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2080;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta653<F: Float>(t91310: F, t26245: F, t80791: F, t26271: F, t80836: F, t1361: F, t22690: F, t22792: F, t5187: F, t1307: F, t7708: F, t80840: F, t90787: F, t80783: F, t22897: F, t6925: F, t26302: F, t80958: F, t22779: F, t26323: F, t1336: F, t242: F, t80901: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91311, t91312, t91323, t91328, t91344) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2080::<F>(t91310, t26245, t80791, t26271, t80836, t1361, t22690, t22792, t5187, t1307, t7708, t80840, t90787);
        let (t91345, t91346, t91351, t91357, t91359, t91361) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2081::<F>(t91344, t26245, t80783, t22897, t6925, t26302, t80958, t22779, t26323, t1336, t242, t80901);
    (t91311, t91312, t91323, t91328, t91345, t91346, t91351, t91357, t91359, t91361)
}
