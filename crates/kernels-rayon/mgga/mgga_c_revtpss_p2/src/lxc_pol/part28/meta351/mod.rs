//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta351(t11200: f64, t225: f64, t127: f64, t3218: f64, t371: f64, t1025: f64, t1058: f64, t3191: f64, t1021: f64, t3201: f64, t3231: f64, t1054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t11940, t11951, t11952, t11954, t11956, t11965, t11967) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1371(t11200, t225, t127, t3218, t371, t1025, t1058, t3191, t1021, t3201, t3231, t1054);
    (t11940, t11951, t11952, t11954, t11956, t11965, t11967)
}
