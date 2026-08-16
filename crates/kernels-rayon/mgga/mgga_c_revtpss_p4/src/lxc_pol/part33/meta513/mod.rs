//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1840;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1841;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta513(t1580: f64, t7014: f64, t689: f64, t27279: f64, t7058: f64, t72: f64, t7769: f64, t686: f64, t25375: f64, t25387: f64, t1559: f64, t886: f64, t25392: f64, t1955: f64, t7057: f64, t14495: f64, t1949: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27334, t27335, t27338, t27340, t27341) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1840(t1580, t7014, t689, t27279, t7058, t72, t7769, t686);
        let (t27342, t27344, t27349, t27350, t27353) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1841(t25375, t27341, t25387, t1559, t886, t25392, t1955, t7057);
        let (t27354, t27357) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1842(t14495, t25392, t1949, t2718);
    (t27334, t27335, t27338, t27340, t27341, t27342, t27344, t27349, t27350, t27353, t27354, t27357)
}
