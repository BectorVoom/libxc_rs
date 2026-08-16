//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1651;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta454<F: Float>(t23095: F, t23105: F, t23107: F, t23140: F, t23143: F, t23100: F, t23114: F, t23117: F, t23119: F, t23125: F, t23128: F, t23130: F, t23134: F, t23136: F, t23147: F, t24217: F, t218: F, t7084: F, t798: F, t23013: F, t23031: F, t2684: F, t7101: F, t2047: F, t2627: F, t2633: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24218, t24220, t24221, t24230, t24231, t24233) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1651::<F>(t23095, t23105, t23107, t23140, t23143, t23100, t23114, t23117, t23119, t23125, t23128, t23130, t23134, t23136, t23147);
        let (t24234, t24235, t24237, t24246, t24250, t24251, t24256) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1652::<F>(t24217, t24233, t218, t7084, t798, t23013, t23031, t2684, t7101, t2047, t2627, t2633);
    (t24218, t24220, t24221, t24230, t24231, t24234, t24235, t24237, t24246, t24250, t24251, t24256)
}
