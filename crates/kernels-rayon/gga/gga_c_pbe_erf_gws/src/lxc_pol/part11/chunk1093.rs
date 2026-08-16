//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1093/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1093(t39951: f64, t30511: f64, t22934: f64, t22939: f64, t3443: f64, t1803: f64, t185: f64, t186: f64, t22968: f64, t3399: f64, t3445: f64, t22982: f64, t22986: f64, t22988: f64, t22994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47527 = 16.0_f64 / 45.0_f64 * t39951;
    let t47528 = 16.0_f64 / 135.0_f64 * t30511;
    let t47529 = 64.0_f64 / 405.0_f64 * t22934;
    let t47530 = 128.0_f64 / 405.0_f64 * t22939;
    let t47531 = t3443 * t3443;
    let t47535 = 4.0_f64 / 5.0_f64 * t185 * t186 * t1803 * t47531;
    let t47536 = 64.0_f64 / 405.0_f64 * t22968;
    let t47538 = 8.0_f64 / 5.0_f64 * t3399 * t3445;
    let t47543 = t47527 - t47528 + t47529 + t47530 + t47535 - t47536 - t47538 + 0.60617527037037037035e-2_f64 * t22982 - 8.0_f64 / 9.0_f64 * t22986 - 0.5402469135802469136e-1_f64 * t22988 + 8.0_f64 / 3.0_f64 * t22994;
    (t47527, t47528, t47529, t47530, t47535, t47536, t47538, t47543)
}
