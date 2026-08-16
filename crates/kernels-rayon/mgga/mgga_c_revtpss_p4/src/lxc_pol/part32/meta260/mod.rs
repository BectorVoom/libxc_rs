//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta260 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1096;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1097;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1098;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1099;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1100;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1101;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta260(t624: f64, t72: f64, t1927: f64, t1923: f64, t2047: f64, t6977: f64, t5: f64, t2048: f64, t6954: f64, t6960: f64, t6963: f64, t7343: f64, t117: f64, t116: f64, t2051: f64, t1310: f64, t2055: f64, t114: f64, t6996: f64, t6999: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7348 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1096(t624, t72);
        let t7349 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1097(t1927, t7348);
        let (t7351, t7352) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1098(t1923, t7349, t2047, t6977);
        let (t7356, t7357, t7359) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1099(t5, t1923, t2048, t6954, t6960, t6963, t7343, t7351, t7352, t117, t116, t2051);
        let t7367 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1100(t1310, t2055);
        let (t7370, t7373) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1101(t114, t6996, t6999);
        let t7374 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1102(t508, t7373);
    (t7348, t7349, t7351, t7352, t7356, t7357, t7359, t7367, t7370, t7373, t7374)
}
