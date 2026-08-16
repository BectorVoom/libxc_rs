//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1894;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1895;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1896;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1897;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta537(t3698: f64, t65: f64, t5047: f64, t1234: f64, t8184: f64, t5362: f64, t7613: f64, t1230: f64, t1256: f64, t8177: f64, t2138: f64, t5261: f64, t8185: f64, t1238: f64, t1791: f64, t26827: f64, t26855: f64, t26863: f64, t29047: f64, t484: f64, t5320: f64, t2137: f64, t5389: f64, t467: f64, t5326: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29054, t29055, t29062) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1894(t3698, t65, t5047, t1234, t8184);
        let (t29065, t29069, t29072, t29074, t29077, t29079) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1895(t5362, t7613, t1230, t8184, t1256, t8177, t2138, t5261, t8185, t1238, t1791, t26827, t26855, t26863, t29047, t29055, t29062, t484, t5320);
        let t29082 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1896(t2137, t5389);
        let t29083 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1897(t29082, t467);
        let t29086 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1898(t2138, t5326);
    (t29054, t29055, t29062, t29065, t29069, t29072, t29074, t29077, t29079, t29082, t29083, t29086)
}
