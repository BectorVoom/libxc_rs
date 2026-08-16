//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta375 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta375(t15125: f64, t15191: f64, t4742: f64, t993: f64, t225: f64, t366: f64, t3224: f64, t4845: f64, t127: f64, t371: f64, t4852: f64, t1025: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15638, t15639, t15654, t15655, t15656, t15662, t15666, t15668) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1412(t15125, t15191, t4742, t993, t225, t366, t3224, t4845, t127, t371, t4852, t1025);
    (t15638, t15639, t15654, t15655, t15656, t15662, t15666, t15668)
}
