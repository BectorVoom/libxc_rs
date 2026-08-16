//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 475/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk475(t1986: f64, t226: f64, t1354: f64, t225: f64, t666: f64, t679: f64, t1626: f64, t1629: f64, t1633: f64, t1637: f64, t1647: f64, t1650: f64, t1654: f64, t1658: f64, t231: f64) -> (f64, f64, f64, f64) {
    let t1988 = 4.0_f64 / 3.0_f64 * t226 * t1986;
    let t1989 = t1354 * t225;
    let t1992 = t666 * t679;
    let t1994 = t1988 + 4.0_f64 / 3.0_f64 * t1989 * t231 + 8.0_f64 / 3.0_f64 * t1992 - t1626 + t1629 + t1633 + t1637 + t1647 + t1650 + t1654 + t1658;
    (t1988, t1989, t1992, t1994)
}
