//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta152 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk823;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk824;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk825;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk826;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk827;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk828;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk829;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta152(t635: f64, t2251: f64, t3360: f64, t128: f64, t2304: f64, t1120: f64, t1121: f64, t2258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3361, t3362) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk823(t635);
        let t3363 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk824(t2251, t3362);
        let (t3364, t3365) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk825(t3360, t3363, t128);
        let t3367 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk826(t2304);
        let t3368 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk827(t2251, t3367);
        let (t3369, t3370) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk828(t1120, t3368, t128);
        let t3372 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk829(t1121, t2258);
        let (t3373, t3374) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk830(t1120, t3372, t128);
    (t3361, t3362, t3363, t3364, t3365, t3367, t3368, t3369, t3370, t3372, t3373, t3374)
}
