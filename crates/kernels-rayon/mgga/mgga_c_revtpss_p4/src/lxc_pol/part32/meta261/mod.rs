//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1103;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta261(t2089: f64, t670: f64, t2061: f64, t212: f64, t780: f64, t689: f64, t2062: f64, t786: f64, t789: f64, t7023: f64, t7031: f64, t7034: f64, t7041: f64, t7026: f64, t7039: f64, t7046: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7378, t7384) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1103(t2089, t670, t2061, t212);
        let (t7385, t7387, t7388, t7390, t7391, t7393, t7394, t7396, t7398) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1104(t7384, t780, t689, t2062, t786, t789, t7023, t7031, t7034, t7041, t7026, t7039, t7046);
    (t7378, t7384, t7385, t7387, t7388, t7390, t7391, t7393, t7394, t7396, t7398)
}
