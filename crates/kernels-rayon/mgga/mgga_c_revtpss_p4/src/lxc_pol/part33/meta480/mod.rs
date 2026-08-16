//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1755;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1756;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta480(t2487: f64, t25245: f64, t2689: f64, t7030: f64, t1945: f64, t2693: f64, t807: f64, t2718: f64, t64: f64, t239: f64, t820: f64, t7036: f64, t843: f64, t839: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25246, t25254, t25255, t25256, t25260) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1755(t2487, t25245, t2689, t7030, t1945, t2693, t807, t2718, t64);
        let (t25262, t25266) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1756(t239, t25260, t820, t7036, t843);
        let (t25267, t25270) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1757(t25266, t839, t241, t7036, t820);
    (t25246, t25254, t25255, t25256, t25260, t25262, t25266, t25267, t25270)
}
