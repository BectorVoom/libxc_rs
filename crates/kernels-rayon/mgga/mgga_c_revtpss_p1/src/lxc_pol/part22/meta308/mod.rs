//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta308(t10175: f64, t3917: f64, t3889: f64, t566: f64, t64: f64, t843: f64, t112: f64, t2289: f64, t666: f64, t2341: f64, t625: f64, t2367: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10176, t10186, t10199, t10201, t10202, t10204, t10206) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1746(t10175, t3917, t3889, t566, t64, t843, t112, t2289, t666, t2341, t625, t2367);
    (t10176, t10186, t10199, t10201, t10202, t10204, t10206)
}
