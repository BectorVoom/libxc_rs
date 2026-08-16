//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2090/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2090(t7150: f64, t99708: f64, t1977: f64, t994: f64, t11627: f64, t1983: f64, t99682: f64, t11223: f64, t7143: f64, t3057: f64, t7810: f64, t11120: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100494 = t7150 * t99708;
    let t100586 = t994 * t1977;
    let t100596 = t1983 * t99682 * t11627;
    let t100658 = t11223 * t7143;
    let t100681 = t3057 * t7810;
    let t100690 = t7143 * t11120;
    (t100494, t100586, t100596, t100658, t100681, t100690)
}
