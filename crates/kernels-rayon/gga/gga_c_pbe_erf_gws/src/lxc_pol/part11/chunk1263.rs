//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1263/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1263(t11459: f64, t13347: f64, t2168: f64, t2170: f64, t11478: f64, t13431: f64, t3138: f64, t3139: f64, t1133: f64, t13578: f64, t2105: f64, t2253: f64, t2343: f64, t2345: f64, t3219: f64, t3257: f64, t343: f64, t3803: f64, t3854: f64, t45974: f64, t45990: f64, t49908: f64, t50049: f64, t50051: f64, t50056: f64, t50073: f64, t816: f64, t9482: f64) -> (f64, f64, f64) {
    let t50077 = t2168 * t2170 * t11459 * t13347 / 8.0_f64;
    let t50087 = 3.0_f64 / 8.0_f64 * t3138 * t3139 * t11478 * t13431;
    let t50088 = -t50049 + t50051 + t50056 - t2253 * t3257 * t3803 * t816 * t3854 * t343 / 64.0_f64 - t2253 * t9482 * t13578 * t2105 * t1133 * t343 / 48.0_f64 - t50073 + t50077 + t2343 * t2345 * t3219 * t49908 / 96.0_f64 - 7.0_f64 / 32.0_f64 * t45974 + 35.0_f64 / 48.0_f64 * t45990 + t50087;
    (t50077, t50087, t50088)
}
