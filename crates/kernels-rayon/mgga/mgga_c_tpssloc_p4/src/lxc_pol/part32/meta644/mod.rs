//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2063;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta644(t22724: f64, t26474: f64, t22751: f64, t26194: f64, t1887: f64, t80830: f64, t26211: f64, t6883: f64, t268: f64, t557: f64, t6559: f64, t26333: f64, t81326: f64, t22642: f64, t22643: f64, t7700: f64, t22674: f64, t26202: f64, t6897: f64, t22716: f64, t7701: f64, t1834: f64, t212: f64, t6890: f64, t26215: f64, t81228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90582, t90585, t90591, t90605, t90607, t90609) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2063(t22724, t26474, t22751, t26194, t1887, t80830, t26211, t6883, t268, t557, t6559, t26333, t81326);
        let (t90642, t90646, t90659, t90663, t90686) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2064(t22642, t22643, t7700, t22674, t26202, t6897, t22716, t7701, t1834, t212, t6890, t26215, t81228, t81326);
    (t90582, t90585, t90591, t90605, t90607, t90609, t90642, t90646, t90659, t90663, t90686)
}
