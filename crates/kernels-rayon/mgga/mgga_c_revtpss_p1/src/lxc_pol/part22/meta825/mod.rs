//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta825 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2943;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta825(t5552: f64, t588: f64, t5560: f64, t13581: f64, t177: f64, t762: f64, t1317: f64, t13632: f64, t3857: f64, t5569: f64, t512: f64, t749: f64, t5567: f64, t13672: f64, t2608: f64, t5566: f64, t1856: f64, t9544: f64, t13597: f64, t2516: f64, t2626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48185, t48212, t48222, t48225, t48227, t48230) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2943(t5552, t588, t5560, t13581, t177, t762, t1317, t13632, t3857, t5569, t512, t749);
        let (t48235, t48237, t48240, t48243, t48255, t48260) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2944(t3857, t5567, t1317, t13672, t2608, t512, t5566, t1856, t9544, t13597, t2516, t2626);
    (t48185, t48212, t48222, t48225, t48227, t48230, t48235, t48237, t48240, t48243, t48255, t48260)
}
