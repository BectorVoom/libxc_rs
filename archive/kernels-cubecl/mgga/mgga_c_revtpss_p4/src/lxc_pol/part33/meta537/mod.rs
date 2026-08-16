//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1894;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1895;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1896;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1897;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta537<F: Float>(t3698: F, t65: F, t5047: F, t1234: F, t8184: F, t5362: F, t7613: F, t1230: F, t1256: F, t8177: F, t2138: F, t5261: F, t8185: F, t1238: F, t1791: F, t26827: F, t26855: F, t26863: F, t29047: F, t484: F, t5320: F, t2137: F, t5389: F, t467: F, t5326: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29054, t29055, t29062) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1894::<F>(t3698, t65, t5047, t1234, t8184);
        let (t29065, t29069, t29072, t29074, t29077, t29079) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1895::<F>(t5362, t7613, t1230, t8184, t1256, t8177, t2138, t5261, t8185, t1238, t1791, t26827, t26855, t26863, t29047, t29055, t29062, t484, t5320);
        let t29082 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1896::<F>(t2137, t5389);
        let t29083 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1897::<F>(t29082, t467);
        let t29086 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1898::<F>(t2138, t5326);
    (t29054, t29055, t29062, t29065, t29069, t29072, t29074, t29077, t29079, t29082, t29083, t29086)
}
