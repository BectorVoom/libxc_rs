//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1008;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1009;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta220(t5: f64, t1497: f64, t2247: f64, t4173: f64, t5812: f64, t5816: f64, t5872: f64, t603: f64, t91: f64, t117: f64, t1518: f64, t94: f64, t1843: f64, t1513: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t5876, t5877) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1008(t5, t1497, t2247, t4173, t5812, t5816, t5872, t603, t91, t117);
        let t5883 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1009(t1518);
        let (t5884, t5887, t5891) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1010(t5883, t94, t1518, t1843, t1513);
    (t5876, t5877, t5883, t5884, t5887, t5891)
}
