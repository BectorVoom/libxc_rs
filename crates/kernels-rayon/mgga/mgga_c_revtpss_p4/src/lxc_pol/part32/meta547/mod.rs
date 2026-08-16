//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta547(t26069: f64, t96259: f64, t26230: f64, t9685: f64, t25878: f64, t2470: f64, t26270: f64, t7284: f64, t96220: f64, t9675: f64, t94771: f64, t7514: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96260, t96264, t96265, t96276, t96277, t96279, t96280, t96282) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1862(t26069, t96259, t26230, t9685, t25878, t2470, t26270, t7284, t96220, t9675, t94771, t7514, t9288);
    (t96260, t96264, t96265, t96276, t96277, t96279, t96280, t96282)
}
