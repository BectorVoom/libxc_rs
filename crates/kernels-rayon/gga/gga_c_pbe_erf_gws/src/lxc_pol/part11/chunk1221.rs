//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1221/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1221(t44695: f64, t11600: f64, t11925: f64, t2312: f64, t2343: f64, t2345: f64, t3257: f64, t36626: f64, t3717: f64, t3803: f64, t3814: f64, t44282: f64, t44763: f64, t49371: f64, t49372: f64, t49378: f64, t49382: f64, t49387: f64, t816: f64) -> (f64, f64, f64) {
    let t49388 = 7.0_f64 / 4.0_f64 * t44695;
    let t49399 = 3.0_f64 / 8.0_f64 * t11600 * t11925;
    let t49401 = t49371 + t49372 + 119.0_f64 / 144.0_f64 * t36626 - t49378 + t49382 + t49387 + t49388 + t2343 * t2345 * t44282 * t3814 / 96.0_f64 - t2312 * t3257 * t3803 * t816 * t3717 / 32.0_f64 + t49399 + 7.0_f64 / 24.0_f64 * t44763;
    (t49388, t49399, t49401)
}
