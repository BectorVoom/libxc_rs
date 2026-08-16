//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 894/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk894(t163: f64, t169: f64, t234: f64, t922: f64, t1: f64, t4576: f64, t550: f64, t553: f64, t6: f64, t6045: f64, t153: f64, t413: f64, t7236: f64, t7271: f64) -> (f64, f64, f64, f64, f64) {
    let t18021 = 0.40978489723982440011e0_f64 * t169 * t922 * t234 * t163;
    let t18032 = t4576 * t1;
    let t18035 = 0.79015561315637923528e-2_f64 * t550 * t18032 * t553;
    let t18046 = t6 * t6045;
    let t18049 = 0.17888888888888888889e-1_f64 * t7271 + 0.22252592592592592592e0_f64 * t7236 - 0.7316671043820612376e-1_f64 * t413 + 0.15663796296296296297e-1_f64 * t153 * t18046;
    (t18021, t18032, t18035, t18046, t18049)
}
