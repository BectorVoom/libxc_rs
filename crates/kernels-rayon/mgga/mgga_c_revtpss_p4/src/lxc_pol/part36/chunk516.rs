//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 516/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk516(t2453: f64, t556: f64, t136: f64, t561: f64, t2457: f64, t1426: f64, t786: f64, t1363: f64, t2470: f64, t1362: f64, t1386: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3906 = t2453 * t556;
    let t3907 = t561 * t136;
    let t3908 = t3907 * t2457;
    let t3910 = 0.11565819519348392139e-2_f64 * t3906 * t3908;
    let t3914 = t556 * t1426;
    let t3915 = t786 * t3914;
    let t3920 = t1363 * t2470;
    let t3922 = 0.13009920719177044025e-1_f64 * t1362 * t3920;
    let t3930 = t820 * t1386 * t843;
    (t3906, t3907, t3908, t3910, t3914, t3915, t3920, t3922, t3930)
}
