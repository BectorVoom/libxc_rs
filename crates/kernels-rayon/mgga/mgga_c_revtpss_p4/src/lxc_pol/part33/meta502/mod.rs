//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta502 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1816;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1817;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1818;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1819;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1820;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta502(t1243: f64, t7627: f64, t1032: f64, t1269: f64, t2148: f64, t12626: f64, t2147: f64, t7635: f64, t13181: f64, t473: f64, t2142: f64, t3566: f64, t7642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26931, t26936) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1816(t1243, t7627, t1032, t1269);
        let (t26937, t26948) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1817(t2148, t26936, t12626, t2147);
        let t26949 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1818(t26948, t7635);
        let t26969 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1819(t13181, t473);
        let t26976 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1820(t2142, t3566);
        let (t26979, t26994) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1821(t26936, t7642, t3566, t7635);
    (t26931, t26936, t26937, t26948, t26949, t26969, t26976, t26979, t26994)
}
