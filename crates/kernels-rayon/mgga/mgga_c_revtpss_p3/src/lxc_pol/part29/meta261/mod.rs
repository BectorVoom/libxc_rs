//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta261 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1068;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1069;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1070;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1071;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1072;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1073;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1074;
use chunk7::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta261(t239: f64, t7036: f64, t820: f64, t839: f64, t1946: f64, t846: f64, t233: f64, t64: f64, t857: f64, t1032: f64, t251: f64, t867: f64, t786: f64, t1954: f64, t2452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7038 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1068(t239, t7036, t820);
        let (t7039, t7041, t7043) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1069(t7038, t839, t1946, t846, t233, t64);
        let t7045 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1070(t239, t7043, t820);
        let (t7046, t7056) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1071(t7045, t857, t1032, t251);
        let t7057 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1072(t7056, t867);
        let t7058 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1073(t7057, t786);
        let t7063 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1074(t1954, t2452);
        let t7064 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1075(t7057, t7063);
    (t7038, t7039, t7041, t7043, t7045, t7046, t7056, t7057, t7058, t7063, t7064)
}
