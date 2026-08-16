//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1845;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta464(t15979: f64, t15982: f64, t15984: f64, t15986: f64, t16164: f64, t184: f64, t20396: f64, t17: f64, t12118: f64, t12121: f64, t12123: f64, t12133: f64, t12141: f64, t9853: f64, t9859: f64, t20519: f64, t20521: f64, t20525: f64, t225: f64, t12155: f64, t20356: f64, t5279: f64, t6347: f64, t1347: f64, t20416: f64, t1819: f64, t1821: f64, t5278: f64, t546: f64, t548: f64, t6404: f64, t6408: f64, t6411: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20526, t20527, t20528, t20529, t20530, t20531, t20532, t20533) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1845(t15979, t15982, t15984, t15986, t16164, t184, t20396, t17, t12118, t12121, t12123, t12133, t12141, t9853, t9859);
        let (t20536, t20544, t20547, t20550, t20553) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1846(t20519, t20521, t20525, t20533, t225, t12155, t20356, t5279, t6347, t1347, t20416, t1819, t1821, t5278, t546, t548, t6404, t6408, t6411);
    (t20526, t20527, t20528, t20529, t20530, t20531, t20532, t20536, t20544, t20547, t20550, t20553)
}
