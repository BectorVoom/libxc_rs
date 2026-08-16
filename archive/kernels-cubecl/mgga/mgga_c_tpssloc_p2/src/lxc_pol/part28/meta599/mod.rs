//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1899;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta599<F: Float>(t22751: F, t26190: F, t26356: F, t6914: F, t1385: F, t1992: F, t22635: F, t3886: F, t5353: F, t3888: F, t55118: F, t1799: F, t22633: F, t1887: F, t80827: F, t26334: F, t26339: F, t81159: F, t22716: F, t7697: F, t1307: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90470, t90472, t90477, t90485, t90488) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1899::<F>(t22751, t26190, t26356, t6914, t1385, t1992, t22635, t3886, t5353, t3888, t55118, t1799);
        let (t90491, t90497, t90498, t90500, t90503, t90506) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1900::<F>(t22633, t22635, t3888, t90488, t1887, t80827, t26334, t26339, t81159, t22716, t7697, t1307, t1385);
    (t90470, t90472, t90477, t90485, t90491, t90497, t90498, t90500, t90503, t90506)
}
