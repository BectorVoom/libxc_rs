//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 476/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk476(t225: f64, t2769: f64, t2435: f64, t871: f64, t785: f64, t870: f64, t2439: f64, t123: f64, t212: f64, t676: f64, t822: f64, t251: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2770 = t225 * t2769;
    let t2776 = 0.73171657588172351096e-2_f64 * t2435 * t871;
    let t2777 = t785 * t225;
    let t2778 = t2777 * t870;
    let t2780 = 0.65049603595885220126e-3_f64 * t2439 * t2778;
    let t2782 = t123 * t676 * t212;
    let t2783 = t225 * t822;
    let t2784 = t251 * t836;
    (t2770, t2776, t2777, t2780, t2782, t2783, t2784)
}
