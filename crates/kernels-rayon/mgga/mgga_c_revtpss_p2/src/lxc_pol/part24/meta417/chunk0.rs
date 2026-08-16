//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1363/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1363(t1086: f64, t11200: f64, t3090: f64, t16565: f64, t994: f64, t42859: f64, t42862: f64, t342: f64, t3145: f64, t368: f64, t42871: f64) -> (f64, f64, f64, f64) {
    let t43291 = t11200 * t1086 * t3090;
    let t43341 = t994 * t16565;
    let t43346 = t42859 * t42862;
    let t43347 = t342 * t43346;
    let t43350 = 1.0_f64 / t3145 / t368;
    let t43351 = t42871 * t43350;
    (t43291, t43341, t43347, t43351)
}
