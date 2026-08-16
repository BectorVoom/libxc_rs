//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1258;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1259;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta282(t2514: f64, t2596: f64, t746: f64, t1340: f64, t2491: f64, t2495: f64, t744: f64, t215: f64, t681: f64, t268: f64, t702: f64, t2564: f64, t2567: f64, t675: f64, t30: f64, t525: f64, t2: f64, t22: f64, t33: f64, t527: f64, t2490: f64, t737: f64, t2492: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9318, t9320, t9323, t9325, t9329) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1258(t2514, t2596, t746, t1340, t2491, t2495, t744, t215, t681, t268, t702);
        let t9333 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1259(t2564, t2567, t268, t675);
        let (t9335, t9342, t9350, t9367, t9368) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1260(t30, t525, t2, t22, t33, t527, t2490, t737, t2492, t744);
    (t9318, t9320, t9323, t9325, t9329, t9333, t9335, t9342, t9350, t9367, t9368)
}
