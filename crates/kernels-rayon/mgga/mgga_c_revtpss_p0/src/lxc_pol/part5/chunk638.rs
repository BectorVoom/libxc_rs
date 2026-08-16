//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 638/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk638(t1419: f64, t212: f64, t1358: f64, t689: f64, t1357: f64, t1445: f64, t2453: f64, t556: f64, t136: f64, t561: f64, t2457: f64, t1420: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3899 = t212 * t1419;
    let t3900 = t3899 * t1358;
    let t3901 = t689 * t3900;
    let t3903 = t1357 * t1445;
    let t3904 = t689 * t3903;
    let t3906 = t2453 * t556;
    let t3907 = t561 * t136;
    let t3908 = t3907 * t2457;
    let t3910 = 0.11565819519348392139e-2_f64 * t3906 * t3908;
    let t3911 = t786 * t1420;
    (t3899, t3900, t3901, t3903, t3904, t3906, t3907, t3908, t3910, t3911)
}
