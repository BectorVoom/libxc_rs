//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1266/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1266(t13293: f64, t39095: f64, t11808: f64, t12054: f64, t11984: f64, t13491: f64, t3180: f64, t45248: f64, t11464: f64, t11514: f64, t11994: f64, t13335: f64, t13340: f64, t13347: f64, t21361: f64, t2255: f64, t2277: f64, t2343: f64, t2345: f64, t3219: f64, t3235: f64, t3247: f64, t46151: f64, t49374: f64, t49853: f64, t50002: f64, t6555: f64, t904: f64, t916: f64, t929: f64) -> (f64, f64, f64, f64, f64) {
    let t50158 = t39095 * t13293 / 16.0_f64;
    let t50160 = t12054 * t11808 / 8.0_f64;
    let t50162 = t11984 * t13491 / 24.0_f64;
    let t50168 = t45248 * t3180 / 12.0_f64;
    let t50181 = 7.0_f64 / 48.0_f64 * t46151 - 3.0_f64 / 64.0_f64 * t6555 * t916 * t904 * t49853 + 35.0_f64 / 128.0_f64 * t929 * t21361 * t904 * t50002 + t50158 - t50160 - t50162 - t2277 * t2255 * t11994 * t13340 / 512.0_f64 - t50168 - t2343 * t3235 * t3219 * t13335 / 384.0_f64 + t2343 * t2345 * t11464 * t13347 / 64.0_f64 - 3.0_f64 / 64.0_f64 * t3247 * t2345 * t11514 * t49374;
    (t50158, t50160, t50162, t50168, t50181)
}
