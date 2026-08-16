//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1259/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1259(t49463: f64, t823: f64, t850: f64, t852: f64, t860: f64, t3703: f64, t1130: f64, t11706: f64, t13086: f64, t13325: f64, t13328: f64, t13331: f64, t21010: f64, t2181: f64, t27917: f64, t3154: f64, t339: f64, t340: f64, t3717: f64, t38451: f64, t3848: f64, t3851: f64, t45901: f64, t48985: f64, t49436: f64, t49955: f64, t6429: f64, t870: f64, t9056: f64) -> (f64, f64, f64) {
    let t49986 = t850 * t49463 * t823 * t852 * t860 / 96.0_f64;
    let t50002 = t3703 * t3703;
    let t50018 = -48.0_f64 * t1130 * t13086 * t2181 - 360.0_f64 * t21010 * t339 * t50002 - 36.0_f64 * t2181 * t339 * t49955 - t339 * t340 * t49436 + 3.0_f64 * t339 * t48985 * t870 + 360.0_f64 * t3717 * t3848 * t6429 + 12.0_f64 * t1130 * t45901 + 18.0_f64 * t11706 * t3851 + 240.0_f64 * t13325 * t27917 - 144.0_f64 * t13328 * t9056 + 12.0_f64 * t13331 * t3154 - 72.0_f64 * t38451 * t3848;
    (t49986, t50002, t50018)
}
