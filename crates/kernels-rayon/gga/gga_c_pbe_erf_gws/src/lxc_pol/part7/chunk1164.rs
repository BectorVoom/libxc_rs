//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1164/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1164(t6377: f64, t6627: f64, t20578: f64, t2168: f64, t2170: f64, t875: f64, t2164: f64, t6442: f64, t20182: f64, t20761: f64, t20769: f64, t20781: f64, t20785: f64, t2272: f64, t2338: f64, t2343: f64, t2345: f64, t3235: f64, t3247: f64, t6282: f64, t6360: f64, t6579: f64, t6580: f64) -> (f64, f64, f64) {
    let t20786 = t6627 * t6377;
    let t20791 = t2168 * t2170 * t20578 * t875 / 12.0_f64;
    let t20792 = t2164 * t6442;
    let t20793 = 7.0_f64 / 72.0_f64 * t20792;
    let t20794 = t20761 + 5.0_f64 / 64.0_f64 * t6579 * t6580 * t2338 + 5.0_f64 / 64.0_f64 * t6579 * t6580 * t2272 - t20769 + 9.0_f64 / 256.0_f64 * t3247 * t3235 * t6282 * t6360 + t2343 * t2345 * t20182 * t875 / 96.0_f64 + t20781 - t20785 - 7.0_f64 / 48.0_f64 * t20786 + t20791 + t20793;
    (t20791, t20793, t20794)
}
