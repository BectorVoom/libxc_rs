//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta326(t1788: f64, t2221: f64, t2223: f64, t11987: f64, t1408: f64, t2: f64, t3704: f64, t12000: f64, t1649: f64, t3711: f64, t225: f64, t5213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15984, t15986, t15989, t15992, t16003, t16006, t16022) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1512(t1788, t2221, t2223, t11987, t1408, t2, t3704, t12000, t1649, t3711, t225, t5213);
    (t15984, t15986, t15989, t15992, t16003, t16006, t16022)
}
