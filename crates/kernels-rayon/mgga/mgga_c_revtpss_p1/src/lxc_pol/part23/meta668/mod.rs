//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2401;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta668(t1063: f64, t247: f64, t42778: f64, t906: f64, t373: f64, t675: f64, t828: f64, t3046: f64, t3316: f64, t4891: f64, t11238: f64, t196: f64) -> (f64, f64, f64, f64, f64) {
        let (t42781, t42792, t42793) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2401(t1063, t247, t42778, t906, t373, t675, t828);
        let (t42830, t42859) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2402(t3046, t3316, t4891, t11238, t196);
    (t42781, t42792, t42793, t42830, t42859)
}
