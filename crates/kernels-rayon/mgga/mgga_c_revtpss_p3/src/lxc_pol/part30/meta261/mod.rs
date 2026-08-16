//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1147;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1148;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1149;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1150;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1151;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1152;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1153;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta261(t1032: f64, t251: f64, t867: f64, t786: f64, t1958: f64, t72: f64, t686: f64, t1954: f64, t2452: f64, t1955: f64, t860: f64, t233: f64, t2769: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7056 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1147(t1032, t251);
        let t7057 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1148(t7056, t867);
        let t7058 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1149(t7057, t786);
        let (t7059, t7060) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1150(t1958, t72, t686);
        let (t7062, t7063) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1151(t7058, t7060, t1954, t2452);
        let t7064 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1152(t7057, t7063);
        let (t7066, t7067, t7070) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1153(t7060, t7064, t1955, t860, t7056);
        let t7071 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1154(t233, t2769);
    (t7056, t7057, t7058, t7059, t7060, t7062, t7063, t7064, t7066, t7067, t7070, t7071)
}
