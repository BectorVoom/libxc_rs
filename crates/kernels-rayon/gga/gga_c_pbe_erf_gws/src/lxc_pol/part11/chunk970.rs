//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 970/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk970(t1672: f64, t185: f64, t3455: f64, t10742: f64, t586: f64, t1620: f64, t16904: f64, t3402: f64, t1006: f64, t7121: f64, t1764: f64, t3345: f64) -> (f64, f64, f64, f64, f64) {
    let t30593 = t185 * t1672 * t3455;
    let t30630 = t10742 * t586;
    let t30660 = t1620 * t16904 * t3402;
    let t30666 = t1006 * t7121;
    let t30740 = t3345 * t1764;
    (t30593, t30630, t30660, t30666, t30740)
}
