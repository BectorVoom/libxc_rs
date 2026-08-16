//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta342 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1145;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta342(t1646: f64, t3056: f64, t225: f64, t3106: f64, t4817: f64, t11710: f64, t4787: f64, t3091: f64, t245: f64, t4890: f64, t3088: f64, t3317: f64, t1065: f64, t1668: f64, t372: f64, t4823: f64, t1087: f64, t11773: f64, t1062: f64, t4857: f64, t11986: f64, t1592: f64, t247: f64, t1063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15669, t15670, t15675, t15684, t15687, t15688, t15689) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1145(t1646, t3056, t225, t3106, t4817, t11710, t4787, t3091, t245, t4890, t3088, t3317);
        let (t15691, t15696, t15700, t15707, t15712) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1146(t1065, t1668, t372, t4823, t1087, t11773, t1062, t4857, t11986, t1592, t247, t1063);
    (t15669, t15670, t15675, t15684, t15687, t15688, t15689, t15691, t15696, t15700, t15707, t15712)
}
