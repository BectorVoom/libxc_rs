//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta140 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk935;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk936;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk937;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk938;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk939;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk940;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk941;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk942;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk943;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta140(t2257: f64, t1941: f64, t268: f64, t404: f64, t1123: f64, t689: f64, t1263: f64, t159: f64, t635: f64, t2251: f64, t128: f64, t2304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3351 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk935(t2257);
        let t3356 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk936(t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk937(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk938(t1263, t159);
        let t3361 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk939(t635);
        let t3362 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk940(t3361);
        let t3363 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk941(t2251, t3362);
        let (t3364, t3365) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk942(t3360, t3363, t128);
        let t3367 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk943(t2304);
        let t3368 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk944(t2251, t3367);
    (t3351, t3356, t3357, t3358, t3360, t3361, t3362, t3363, t3364, t3365, t3367, t3368)
}
