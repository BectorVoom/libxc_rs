//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1039/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1039(t119989: f64, t119839: f64, t119968: f64, t2470: f64, t31780: f64, t31784: f64, t31805: f64, t860: f64, t817: f64, t8485: f64, t2718: f64, t8479: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119990 = 0.3526350471130277186e-3_f64 * t119989;
    let t119991 = t119968 * t119839;
    let t119993 = t31780 * t2470;
    let t119995 = 0.34270468708064099208e-1_f64 * t31784 * t119993;
    let t120000 = t31805 * t860;
    let t120002 = t120000 * t8485 * t817;
    let t120004 = t8479 * t2718;
    (t119990, t119991, t119993, t119995, t120002, t120004)
}
