//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2060;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta590(t94570: f64, t1445: f64, t2439: f64, t25916: f64, t1358: f64, t212: f64, t26034: f64, t689: f64, t25877: f64, t94390: f64, t94385: f64, t9675: f64, t7289: f64, t94377: f64, t122: f64, t72: f64, t7274: f64, t3916: f64, t25895: f64, t7285: f64, t9288: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94571, t94580, t94584, t94589, t94590) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2060(t94570, t1445, t2439, t25916, t1358, t212, t26034, t689, t25877, t94390, t94385, t9675);
        let (t94591, t94593, t94596, t94597, t94598, t94600, t94602) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2061(t94589, t94590, t7289, t94377, t122, t72, t7274, t3916, t25895, t7285, t9288, t7284);
    (t94571, t94580, t94584, t94589, t94590, t94591, t94593, t94596, t94597, t94598, t94600, t94602)
}
