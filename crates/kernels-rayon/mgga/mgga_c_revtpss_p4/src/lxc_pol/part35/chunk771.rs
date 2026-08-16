//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 771/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk771(t11735: f64, t345: f64, t1014: f64, t2852: f64, t245: f64, t3089: f64, t3088: f64, t3114: f64, t271: f64, t2857: f64, t11144: f64, t11150: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11737 = 5.0_f64 / 1296.0_f64 * t345 * t11735;
    let t11765 = t1014 * t2852;
    let t11772 = t3089 * t245;
    let t11773 = t3088 * t11772;
    let t11774 = t3114 * t11773;
    let t11821 = 1.0_f64 / t271 / t2857;
    let t11822 = t11821 * t11144;
    let t11827 = t3252 * t11150;
    (t11737, t11765, t11772, t11774, t11822, t11827)
}
