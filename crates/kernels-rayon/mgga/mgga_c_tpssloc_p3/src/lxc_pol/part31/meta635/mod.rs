//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1899;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta635(t22633: f64, t26338: f64, t90566: f64, t22751: f64, t28213: f64, t28210: f64, t28233: f64, t6883: f64, t22674: f64, t28232: f64, t6897: f64, t28195: f64, t22635: f64, t26337: f64, t5353: f64, t5325: f64, t90488: f64, t1307: f64, t567: f64, t6330: f64, t90591: f64, t28199: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97527, t97529, t97537, t97548, t97571, t97573) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1899(t22633, t26338, t90566, t22751, t28213, t28210, t28233, t6883, t22674, t28232, t6897, t28195);
        let (t97577, t97583, t97588, t97599) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1900(t22633, t22635, t26337, t5353, t5325, t90488, t1307, t567, t6330, t90591, t28199, t6897, t794);
    (t97527, t97529, t97537, t97548, t97571, t97573, t97577, t97583, t97588, t97599)
}
