//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 858/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk858(t7268: f64, t7312: f64, t650: f64, t186: f64, t211: f64, t2730: f64, t2737: f64, t1024: f64, t5343: f64, t5205: f64, t7184: f64, t7185: f64, t7187: f64, t7190: f64, t7193: f64, t7198: f64, t7203: f64, t7208: f64, t7215: f64, t7221: f64, t7223: f64, t7228: f64, t7230: f64) -> (f64, f64, f64, f64) {
    let t7313 = t7268 + t7312;
    let t7314 = t650 * t7313;
    let t7315 = t186 * t7314;
    let t7317 = 2.0_f64 / 15.0_f64 * t211 * t7315;
    let t7319 = 8.0_f64 / 15.0_f64 * t2730 * t2737;
    let t7321 = 4.0_f64 / 15.0_f64 * t5343 * t1024;
    let t7322 = t7184 + t7185 + 4.0_f64 / 135.0_f64 * t5205 + t7187 - t7190 + t7193 - t7198 + t7203 + t7208 - t7215 + t7221 + t7223 - t7228 + t7230 - t7317 - t7319 + t7321;
    (t7317, t7319, t7321, t7322)
}
