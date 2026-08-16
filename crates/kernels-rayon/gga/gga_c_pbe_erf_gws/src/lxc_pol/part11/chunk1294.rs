//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1294/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1294(t3931: f64, t18850: f64, t18853: f64, t18863: f64, t18920: f64, t18924: f64, t18933: f64, t19517: f64, t19525: f64, t21885: f64, t2429: f64, t321: f64, t382: f64, t48497: f64, t48498: f64, t48499: f64, t49423: f64, t49425: f64, t49426: f64, t49955: f64) -> f64 {
    let t50755 = t3931 * t3931;
    let t50759 = -6.0_f64 * t21885 * t321 * t50755 + 18.0_f64 * t2429 * t382 * t49955 + t18850 + t18853 - t18863 + t18920 + t18924 - t18933 - t19517 + t19525 - t48497 - t48498 + t48499 - t49423 - t49425 - t49426;
    t50759
}
