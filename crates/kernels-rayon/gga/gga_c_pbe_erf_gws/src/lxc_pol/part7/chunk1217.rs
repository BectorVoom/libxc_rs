//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1217/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1217(t2142: f64, t6612: f64, t2083: f64, t2084: f64, t21570: f64, t21577: f64, t21580: f64, t21581: f64, t21586: f64, t21594: f64, t21596: f64, t21600: f64, t21601: f64, t21605: f64, t2253: f64, t2312: f64, t2343: f64, t3257: f64, t6195: f64, t6275: f64, t821: f64, t904: f64, t9343: f64) -> (f64, f64) {
    let t21607 = t6612 * t2142;
    let t21608 = 7.0_f64 / 72.0_f64 * t21607;
    let t21609 = -t6275 * t904 * t821 * t2083 * t21570 / 16.0_f64 + t21577 + t21580 - t2253 * t3257 * t2084 * t21581 / 64.0_f64 + t2312 * t3257 * t6195 * t21586 / 16.0_f64 - t21594 + t21596 - t21600 - 5.0_f64 / 32.0_f64 * t2343 * t9343 * t21601 - 7.0_f64 / 576.0_f64 * t21605 - t21608;
    (t21608, t21609)
}
