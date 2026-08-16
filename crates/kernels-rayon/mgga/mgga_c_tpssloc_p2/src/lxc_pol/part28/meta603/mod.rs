//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1907;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta603(t268: f64, t557: f64, t6559: f64, t26333: f64, t81326: f64, t22633: f64, t26338: f64, t80650: f64, t1985: f64, t22934: f64, t26193: f64, t16413: f64, t214: f64, t225: f64, t567: f64, t22635: f64, t26214: f64, t26331: f64, t3734: f64, t22666: f64, t26202: f64, t22642: f64, t22643: f64, t7700: f64, t22674: f64, t6897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90607, t90609, t90612, t90615, t90626) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1907(t268, t557, t6559, t26333, t81326, t22633, t26338, t80650, t1985, t22934, t26193, t16413, t214, t225, t567);
        let (t90634, t90639, t90642, t90645) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1908(t22635, t26214, t26331, t3734, t1985, t22666, t26202, t22642, t22643, t7700, t22674, t6897);
    (t90607, t90609, t90612, t90615, t90626, t90634, t90639, t90642, t90645)
}
