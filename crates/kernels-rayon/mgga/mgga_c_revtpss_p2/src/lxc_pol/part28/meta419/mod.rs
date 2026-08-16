//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1586;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta419(t11710: f64, t4787: f64, t3091: f64, t245: f64, t4890: f64, t3088: f64, t3317: f64, t1065: f64, t1668: f64, t372: f64, t12131: f64, t3095: f64, t4823: f64, t3096: f64, t1087: f64, t11773: f64, t4801: f64, t4181: f64, t4786: f64, t1062: f64, t4857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15682, t15684, t15687, t15688, t15689, t15691, t15692) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1586(t11710, t4787, t3091, t245, t4890, t3088, t3317, t1065, t1668, t372, t12131, t3095);
        let (t15693, t15697, t15700, t15702, t15703, t15707) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1587(t15691, t15692, t372, t4823, t3096, t1087, t11773, t4801, t4181, t4786, t1062, t4857);
    (t15682, t15684, t15687, t15688, t15689, t15691, t15693, t15697, t15700, t15702, t15703, t15707)
}
