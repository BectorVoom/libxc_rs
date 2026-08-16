//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2202;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta634(t1936: f64, t98487: f64, t27123: f64, t7002: f64, t13514: f64, t93: f64, t101469: f64, t1312: f64, t28219: f64, t25832: f64, t7889: f64, t10416: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t101519, t101521, t101524, t101526, t101528, t101530, t101532) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2202(t1936, t98487, t27123, t7002, t13514, t93, t101469, t1312, t28219, t25832, t7889, t10416, t7741);
    (t101519, t101521, t101524, t101526, t101528, t101530, t101532)
}
