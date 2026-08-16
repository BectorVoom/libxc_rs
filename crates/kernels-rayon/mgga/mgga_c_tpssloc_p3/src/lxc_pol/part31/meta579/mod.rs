//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1816;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta579(t26197: f64, t80670: f64, t1834: f64, t213: f64, t225: f64, t80711: f64, t22724: f64, t26474: f64, t22751: f64, t26194: f64, t1887: f64, t80830: f64, t26211: f64, t6883: f64, t268: f64, t557: f64, t6559: f64, t26333: f64, t81326: f64, t80722: f64, t22642: f64, t22643: f64, t7700: f64, t22674: f64, t26202: f64, t6897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90551, t90566, t90581, t90582, t90584, t90591) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1816(t26197, t80670, t1834, t213, t225, t80711, t22724, t26474, t22751, t26194, t1887, t80830);
        let (t90604, t90607, t90609, t90617, t90642, t90645) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1817(t26211, t6883, t268, t557, t6559, t26333, t81326, t80722, t22642, t22643, t7700, t22674, t26202, t6897);
    (t90551, t90566, t90581, t90582, t90584, t90591, t90604, t90607, t90609, t90617, t90642, t90645)
}
