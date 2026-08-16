//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1944;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta622(t1361: f64, t16153: f64, t26288: f64, t1339: f64, t16206: f64, t6936: f64, t1825: f64, t22827: f64, t3719: f64, t1307: f64, t7708: f64, t80840: f64, t90787: f64, t26245: f64, t80783: f64, t22897: f64, t6925: f64, t12369: f64, t1351: f64, t26243: f64, t26302: f64, t80958: f64, t22779: f64, t26323: f64, t1336: f64, t242: f64, t80901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91333, t91336, t91340, t91344) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1944(t1361, t16153, t26288, t1339, t16206, t6936, t1825, t22827, t3719, t1307, t7708, t80840, t90787);
        let (t91346, t91354, t91356, t91358, t91361) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1945(t26245, t80783, t22897, t6925, t12369, t1351, t26243, t26302, t80958, t22779, t26323, t1336, t242, t80901);
    (t91333, t91336, t91340, t91344, t91346, t91354, t91356, t91358, t91361)
}
