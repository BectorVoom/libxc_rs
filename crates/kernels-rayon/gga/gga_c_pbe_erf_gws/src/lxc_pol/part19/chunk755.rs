//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 755/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk755(t1294: f64, t174: f64, t4715: f64, t1258: f64, t155: f64, t331: f64, t434: f64, t456: f64, t4607: f64, t1318: f64, t448: f64, t75: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4717 = t174 * t4715 * t1294;
    let t4718 = 0.85917146441092277512e0_f64 * t4717;
    let t4719 = t155 * t1258;
    let t4723 = t331 * t434;
    let t4730 = t4607 * t456;
    let t4734 = 1.0_f64 / t1318 / t448;
    let t4735 = t75 * t4734;
    (t4718, t4719, t4723, t4730, t4734, t4735)
}
