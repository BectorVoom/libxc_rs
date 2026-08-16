//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1700;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1701;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta403(t15395: f64, t18206: f64, t15338: f64, t4904: f64, t3447: f64, t3431: f64, t6126: f64, t1174: f64, t6130: f64, t11539: f64, t6119: f64, t4889: f64, t4896: f64, t18215: f64, t4900: f64, t11570: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18443, t18446, t18447, t18451, t18452, t18454, t18455, t18457, t18458, t18460) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1700(t15395, t18206, t15338, t4904, t3447, t3431, t6126, t1174, t6130, t11539, t6119, t4889, t4896);
        let (t18466, t18469) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1701(t18215, t4900, t11570, t5392);
    (t18443, t18446, t18447, t18451, t18452, t18454, t18455, t18457, t18458, t18460, t18466, t18469)
}
