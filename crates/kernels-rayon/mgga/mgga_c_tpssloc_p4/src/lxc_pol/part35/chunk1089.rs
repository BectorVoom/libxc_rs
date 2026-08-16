//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1089/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1089(t22298: f64, t475: f64, t1214: f64, t248: f64, t11721: f64, t3508: f64, t11678: f64, t11692: f64, t11719: f64, t11728: f64, t11738: f64, t15438: f64, t15737: f64, t15754: f64, t1737: f64, t1748: f64, t19047: f64, t19051: f64, t19083: f64, t19090: f64, t19096: f64, t22104: f64, t22271: f64, t22275: f64, t22280: f64, t22284: f64, t22288: f64, t3506: f64, t3515: f64, t3577: f64, t467: f64, t5005: f64, t5024: f64, t6207: f64, t6211: f64, t6227: f64, t6232: f64) -> (f64, f64, f64, f64) {
    let t22299 = t22298 * t475;
    let t22301 = t248 * t1214 * t22299;
    let t22307 = t22298 * t11721;
    let t22309 = t248 * t1214 * t22307;
    let t22312 = t22298 * t3508;
    let t22314 = t248 * t1214 * t22312;
    let t22325 = t19083 * t1748 / 144.0_f64 + t3506 * t22271 / 512.0_f64 - t3515 * t22275 / 1024.0_f64 + t15754 / 432.0_f64 - t11678 * t22280 / 768.0_f64 + t11692 * t22284 / 1536.0_f64 - t3577 * t22288 / 768.0_f64 + 11.0_f64 / 108.0_f64 * t19090 + t15737 * t6227 / 512.0_f64 - t15438 * t6232 / 1024.0_f64 - t5005 * t6207 / 1536.0_f64 + t11738 * t22301 / 3072.0_f64 + t5024 * t6207 / 288.0_f64 - t19096 / 1536.0_f64 + t11719 * t22309 / 512.0_f64 - t11728 * t22314 / 512.0_f64 - t5005 * t6211 / 768.0_f64 - 77.0_f64 / 162.0_f64 * t22104 * t467 + t19047 * t1737 / 1024.0_f64 - t19051 * t1748 / 1536.0_f64;
    (t22301, t22309, t22314, t22325)
}
