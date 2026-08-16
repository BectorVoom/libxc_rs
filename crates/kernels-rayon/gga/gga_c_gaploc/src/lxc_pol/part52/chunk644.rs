//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 644/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk644(t11997: f64, t11998: f64, t12011: f64, t12028: f64, t3699: f64, t501: f64, t3718: f64, t605: f64, t12007: f64, t549: f64, t10309: f64, t10313: f64, t10317: f64, t10321: f64, t10323: f64, t10326: f64, t10329: f64, t1429: f64, t9265: f64, t9270: f64, t9276: f64, t9280: f64, t9289: f64, t9296: f64, t9307: f64) -> (f64, f64, f64, f64) {
    let t12030 = t11997 + t11998 + t12011 + t12028;
    let t12032 = t3699 * t501;
    let t12035 = t3718 * t605;
    let t12038 = t549 * t12007;
    let t12043 = 0.39722766613167140743e-1_f64 * t1429 * t12038 - 0.76685851907841499354e0_f64 * t9265 + t9270 - t9276 - t10309 - t10313 - t10317 - t10321 + t10323 - 0.38342925953920749677e0_f64 * t9280 + t9289 + t9296 - t9307 - t10326 + t10329;
    (t12030, t12032, t12035, t12043)
}
