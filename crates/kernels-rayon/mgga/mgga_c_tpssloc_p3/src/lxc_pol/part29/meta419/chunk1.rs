//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1694/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1694(t1351: f64, t1824: f64, t3807: f64, t16305: f64, t3792: f64, t1307: f64, t12345: f64, t1831: f64, t12429: f64, t1354: f64, t16257: f64, t16261: f64, t16265: f64, t16269: f64, t16271: f64, t16275: f64, t16278: f64, t16285: f64, t16290: f64, t16294: f64, t16296: f64, t16300: f64, t3733: f64, t3783: f64, t3795: f64, t3803: f64, t3853: f64, t3858: f64, t3872: f64, t5235: f64, t5240: f64, t5246: f64, t5293: f64, t5310: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16306 = t1824 * t1351;
    let t16307 = t16306 * t3807;
    let t16308 = t16305 * t16307;
    let t16311 = t1824 * t3792;
    let t16312 = t1351 * t1307;
    let t16313 = t16311 * t16312;
    let t16314 = t16305 * t16313;
    let t16317 = t12345 * t1831;
    let t16319 = -t12429 * t5293 / 1536.0_f64 + t5246 * t16257 / 768.0_f64 + t5246 * t16261 / 1536.0_f64 - t3803 * t16265 / 3072.0_f64 - t16269 - t3803 * t16271 / 1536.0_f64 - t3803 * t16275 / 3072.0_f64 - t16278 * t1354 / 1536.0_f64 - t5235 * t3853 / 3072.0_f64 + 5.0_f64 / 384.0_f64 * t3783 * t5310 + t16285 * t3795 / 1536.0_f64 + t16290 - t5235 * t3858 / 3072.0_f64 - t16294 + t3733 * t16296 / 8.0_f64 + t3733 * t16300 / 16.0_f64 + 5.0_f64 / 768.0_f64 * t5240 * t3872 + t3803 * t16308 / 384.0_f64 - t5246 * t16314 / 192.0_f64 - 119.0_f64 / 3456.0_f64 * t16317;
    (t16306, t16307, t16308, t16311, t16312, t16313, t16314, t16319)
}
