//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1131;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1132;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1133;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1134;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1135;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1136;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta255(t3: f64, t5789: f64, param_d: f64, t116: f64, t1518: f64, t670: f64, t117: f64, t4292: f64, t1459: f64, t1461: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t2242: f64, t38: f64, t1925: f64, t2247: f64, t644: f64, t84: f64, t77: f64, t603: f64, t607: f64, t43: f64, t48: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5790, t5795) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1131(t3, t5789, param_d);
        let (t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1132(t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, t5795);
        let (t6954, t6957) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1133(t2242, t38, t1925);
        let t6958 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1134(t2247, t6957);
        let t6960 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1135(t644, t84, t77);
        let t6963 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1136(t603, t607);
        let t6968 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1137(t43, t48);
    (t5790, t5795, t5801, t5802, t5805, t5808, t6954, t6957, t6958, t6960, t6963, t6968)
}
