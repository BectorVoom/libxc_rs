//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 408/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk408(t1413: f64, t1697: f64, t625: f64, t11: f64, t1416: f64, t626: f64, t191: f64, t299: f64, t190: f64, t212: f64, t401: f64, t658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1698 = t1697 * t1413;
    let t1699 = t625 * t1698;
    let t1700 = t11 * t1699;
    let t1702 = t626 * t1416;
    let t1703 = t625 * t1702;
    let t1704 = t11 * t1703;
    let t1706 = t299 * t191;
    let t1709 = 0.11111111111111111111e-1_f64 * t190 * t1706 * t212;
    let t1710 = t401 * t658;
    (t1698, t1699, t1700, t1702, t1703, t1704, t1706, t1709, t1710)
}
