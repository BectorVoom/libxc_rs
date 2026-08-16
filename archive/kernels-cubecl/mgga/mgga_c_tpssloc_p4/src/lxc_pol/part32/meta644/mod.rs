//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2063;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta644<F: Float>(t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F, t26211: F, t6883: F, t268: F, t557: F, t6559: F, t26333: F, t81326: F, t22642: F, t22643: F, t7700: F, t22674: F, t26202: F, t6897: F, t22716: F, t7701: F, t1834: F, t212: F, t6890: F, t26215: F, t81228: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90582, t90585, t90591, t90605, t90607, t90609) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2063::<F>(t22724, t26474, t22751, t26194, t1887, t80830, t26211, t6883, t268, t557, t6559, t26333, t81326);
        let (t90642, t90646, t90659, t90663, t90686) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2064::<F>(t22642, t22643, t7700, t22674, t26202, t6897, t22716, t7701, t1834, t212, t6890, t26215, t81228, t81326);
    (t90582, t90585, t90591, t90605, t90607, t90609, t90642, t90646, t90659, t90663, t90686)
}
