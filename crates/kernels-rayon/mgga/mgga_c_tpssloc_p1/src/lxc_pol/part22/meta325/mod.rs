//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta325(t12072: f64, t1649: f64, t2: f64, t3672: f64, t1787: f64, t2516: f64, t17: f64, t12120: f64, t2663: f64, t5157: f64, t1788: f64, t2225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15952, t15955, t15971, t15972, t15976, t15979, t15982) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1511(t12072, t1649, t2, t3672, t1787, t2516, t17, t12120, t2663, t5157, t1788, t2225);
    (t15952, t15955, t15971, t15972, t15976, t15979, t15982)
}
