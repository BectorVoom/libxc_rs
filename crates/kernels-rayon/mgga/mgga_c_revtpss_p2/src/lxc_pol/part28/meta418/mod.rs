//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1584;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta418(t15648: f64, t373: f64, t371: f64, t372: f64, t4742: f64, t993: f64, t225: f64, t366: f64, t3224: f64, t4845: f64, t127: f64, t4852: f64, t1025: f64, t1646: f64, t3056: f64, t3106: f64, t4817: f64, t1028: f64, t11644: f64, t11649: f64, t11783: f64, t1665: f64, t3208: f64, t3211: f64, t3220: f64, t4854: f64, t4858: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15651, t15654, t15655, t15656, t15662, t15666) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1584(t15648, t373, t371, t372, t4742, t993, t225, t366, t3224, t4845, t127, t4852);
        let (t15669, t15670, t15676) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1585(t1025, t15666, t1646, t3056, t225, t366, t3106, t4817, t1028, t11644, t11649, t11783, t15651, t15656, t15662, t1665, t3208, t3211, t3220, t3224, t4854, t4858);
    (t15651, t15654, t15655, t15666, t15669, t15670, t15676)
}
