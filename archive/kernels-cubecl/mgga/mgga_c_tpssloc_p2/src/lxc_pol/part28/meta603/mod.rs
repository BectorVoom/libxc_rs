//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1907;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta603<F: Float>(t268: F, t557: F, t6559: F, t26333: F, t81326: F, t22633: F, t26338: F, t80650: F, t1985: F, t22934: F, t26193: F, t16413: F, t214: F, t225: F, t567: F, t22635: F, t26214: F, t26331: F, t3734: F, t22666: F, t26202: F, t22642: F, t22643: F, t7700: F, t22674: F, t6897: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90607, t90609, t90612, t90615, t90626) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1907::<F>(t268, t557, t6559, t26333, t81326, t22633, t26338, t80650, t1985, t22934, t26193, t16413, t214, t225, t567);
        let (t90634, t90639, t90642, t90645) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1908::<F>(t22635, t26214, t26331, t3734, t1985, t22666, t26202, t22642, t22643, t7700, t22674, t6897);
    (t90607, t90609, t90612, t90615, t90626, t90634, t90639, t90642, t90645)
}
