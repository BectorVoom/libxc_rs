//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1899;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta599(t22751: f64, t26190: f64, t26356: f64, t6914: f64, t1385: f64, t1992: f64, t22635: f64, t3886: f64, t5353: f64, t3888: f64, t55118: f64, t1799: f64, t22633: f64, t1887: f64, t80827: f64, t26334: f64, t26339: f64, t81159: f64, t22716: f64, t7697: f64, t1307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90470, t90472, t90477, t90485, t90488) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1899(t22751, t26190, t26356, t6914, t1385, t1992, t22635, t3886, t5353, t3888, t55118, t1799);
        let (t90491, t90497, t90498, t90500, t90503, t90506) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1900(t22633, t22635, t3888, t90488, t1887, t80827, t26334, t26339, t81159, t22716, t7697, t1307, t1385);
    (t90470, t90472, t90477, t90485, t90491, t90497, t90498, t90500, t90503, t90506)
}
