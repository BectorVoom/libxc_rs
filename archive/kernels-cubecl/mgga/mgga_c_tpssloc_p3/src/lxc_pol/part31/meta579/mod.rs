//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1816;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta579<F: Float>(t26197: F, t80670: F, t1834: F, t213: F, t225: F, t80711: F, t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F, t26211: F, t6883: F, t268: F, t557: F, t6559: F, t26333: F, t81326: F, t80722: F, t22642: F, t22643: F, t7700: F, t22674: F, t26202: F, t6897: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90551, t90566, t90581, t90582, t90584, t90591) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1816::<F>(t26197, t80670, t1834, t213, t225, t80711, t22724, t26474, t22751, t26194, t1887, t80830);
        let (t90604, t90607, t90609, t90617, t90642, t90645) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1817::<F>(t26211, t6883, t268, t557, t6559, t26333, t81326, t80722, t22642, t22643, t7700, t22674, t26202, t6897);
    (t90551, t90566, t90581, t90582, t90584, t90591, t90604, t90607, t90609, t90617, t90642, t90645)
}
