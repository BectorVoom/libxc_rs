//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1298/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1298(t1679: f64, t2049: f64, t582: f64, t7682: f64, t1982: f64, t1981: f64, t1993: f64, t19050: f64, t546: f64, t116: f64, t18679: f64, t18363: f64, t5791: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61942 = t1679 * t2049;
    let t62007 = t7682 * t582;
    let t62020 = t1679 * t1982;
    let t62024 = t1981 * t1993;
    let t62171 = t546 * t19050;
    let t62230 = t18679 * t116;
    let t62247 = t18363 * t5791;
    (t61942, t62007, t62020, t62024, t62171, t62230, t62247)
}
