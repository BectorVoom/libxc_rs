//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 905/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk905(t461: f64, t4859: f64, t14: f64, t2: f64, t41: f64, t174: f64, t6045: f64, t413: f64, t4517: f64, t366: f64, t799: f64, t1236: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18478 = t4859 * t461;
    let t18479 = 960.0_f64 * t18478;
    let t18483 = 1.0_f64 / t14 / t2 / t41 / 48.0_f64;
    let t18486 = t18483 * t2 * t6045 * t174;
    let t18488 = t4517 * t413;
    let t18490 = t799 * t366;
    let t18491 = t1236 * t18490;
    (t18479, t18483, t18486, t18488, t18490, t18491)
}
