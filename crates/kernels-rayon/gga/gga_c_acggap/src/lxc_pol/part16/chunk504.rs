//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 504/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk504(t2722: f64, t2823: f64, t60: f64, t40: f64, t883: f64, t912: f64, t2773: f64, t2775: f64, t690: f64, t286: f64, t229: f64, t699: f64) -> (f64, f64, f64, f64) {
    let t2824 = t2722 + t2823;
    let t2825 = t60 * t2824;
    let t2826 = t40 * t2825;
    let t2835 = t883 * t912;
    let t2838 = t2773 * t2775 * t690;
    let t2839 = t286 * t2838;
    let t2840 = 0.10389515463408878255e3_f64 * t2839;
    let t2841 = t229 * t699;
    (t2826, t2835, t2840, t2841)
}
