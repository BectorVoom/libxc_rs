//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta476(t1614: f64, t2942: f64, t11354: f64, t1600: f64, t2881: f64, t11358: f64, t2880: f64, t4606: f64, t918: f64, t2889: f64, t4598: f64, t2897: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15104, t15107, t15108, t15110, t15111, t15113, t15114, t15116, t15118) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2042(t1614, t2942, t11354, t1600, t2881, t11358, t2880, t4606, t918, t2889, t4598, t2897);
    (t15104, t15107, t15108, t15110, t15111, t15113, t15114, t15116, t15118)
}
