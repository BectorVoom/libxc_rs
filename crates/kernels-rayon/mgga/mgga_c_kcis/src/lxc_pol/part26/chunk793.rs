//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 793/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk793(t2389: f64, t2415: f64, t705: f64, t2387: f64, t2388: f64, t78: f64, t686: f64, t60: f64, t2381: f64, t2386: f64, t2390: f64, t2416: f64, t688: f64, t82: f64, t8747: f64, t8949: f64, t8957: f64, t8961: f64, t8965: f64, t9001: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9236 = t2415 * t2389;
    let t9237 = t9236 * t705;
    let t9249 = t2387 * t705;
    let t9251 = 1.0_f64 / t2388 / t78;
    let t9252 = t9249 * t9251;
    let t9260 = t686 * t686;
    let t9261 = 1.0_f64 / t9260;
    let t9262 = t60 * t9261;
    let t9265 = 0.200175e0_f64 * t688 * t9237 + 0.200175e0_f64 * t2381 * t2390 + 0.59694999999999999999e-1_f64 * t8949 - 0.200175e0_f64 * t2381 * t2416 + t8747 * t82 + 0.92858888888888888885e-1_f64 * t8957 + 0.2671335375e-1_f64 * t2386 * t9237 - 0.2671335375e-1_f64 * t2386 * t9252 - 0.10317654320987654321e0_f64 * t8961 - 0.39796666666666666665e-1_f64 * t8965 - 0.13345e0_f64 * t688 * t9252 + 0.99491666666666666664e-2_f64 * t9001 - 0.178244852896875e-2_f64 * t9262 * t9252;
    (t9236, t9249, t9251, t9260, t9261, t9262, t9265)
}
