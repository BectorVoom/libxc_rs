//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta197 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk914;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk915;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk916;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk917;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta197(t1211: f64, t5245: f64, t1209: f64, t1811: f64, t1256: f64, t1804: f64, t1786: f64, t1230: f64, t1803: f64, t225: f64, t5216: f64, t480: f64, t1796: f64, t3172: f64, t1247: f64, t1263: f64, t3367: f64, t4181: f64, t1042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t5246 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk914(t1211, t5245);
        let t5251 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk915(t1209, t1811);
        let (t5254, t5256, t5258, t5261) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk916(t1256, t1804, t1786, t1230, t1803, t225, t5216);
        let (t5262, t5265) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk917(t480, t5261, t1796, t3172);
        let (t5266, t5268, t5269, t5270) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk918(t1247, t5265, t1263, t3367, t4181, t1042);
    (t5246, t5251, t5254, t5256, t5258, t5261, t5262, t5265, t5266, t5268, t5269, t5270)
}
