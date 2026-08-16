//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta339(t3010: f64, t320: f64, t315: f64, t11132: f64, t11337: f64, t963: f64, t3013: f64, t323: f64, t3006: f64, t3014: f64, t2873: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11465, t11466, t11479, t11480, t11506, t11507, t11509, t11524, t11528) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1359(t3010, t320, t315, t11132, t11337, t963, t3013, t323, t3006, t3014, t2873, t910);
    (t11465, t11466, t11479, t11480, t11506, t11507, t11509, t11524, t11528)
}
