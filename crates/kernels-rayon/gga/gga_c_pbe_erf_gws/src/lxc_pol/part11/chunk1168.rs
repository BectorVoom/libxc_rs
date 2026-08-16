//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1168/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1168(t33581: f64, t33583: f64, t22653: f64, t48472: f64, t85: f64, t22655: f64, t42535: f64, t18885: f64, t18941: f64, t18950: f64, t18954: f64, t18956: f64, t18959: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48502 = 72.0_f64 * t33581;
    let t48503 = 192.0_f64 * t33583;
    let t48504 = 0.23392893589820816284e1_f64 * t22653;
    let t48506 = 0.19751789702565206229e-1_f64 * t48472 * t85;
    let t48507 = 144.0_f64 * t22655;
    let t48508 = 0.23392893589820816284e1_f64 * t42535;
    let t48509 = t18941 + t48502 + t48503 - t18950 + t18954 - t48504 + t48506 + t48507 + t18956 + t18885 - t48508 + t18959;
    (t48502, t48503, t48504, t48506, t48507, t48508, t48509)
}
