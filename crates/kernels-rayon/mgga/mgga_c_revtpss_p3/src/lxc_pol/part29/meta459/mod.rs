//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta459 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1708;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta459(t26249: f64, t3908: f64, t7507: f64, t786: f64, t1364: f64, t2097: f64, t3923: f64, t543: f64, t7301: f64, t25937: f64, t7282: f64, t10073: f64, t1426: f64, t2098: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26251, t26252, t26253, t26255, t26257, t26260, t26261, t26263) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1708(t26249, t3908, t7507, t786, t1364, t2097, t3923, t543, t7301, t25937, t7282, t10073);
        let (t26264, t26265) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1709(t1426, t2098, t786);
    (t26251, t26252, t26253, t26255, t26257, t26260, t26261, t26263, t26264, t26265)
}
