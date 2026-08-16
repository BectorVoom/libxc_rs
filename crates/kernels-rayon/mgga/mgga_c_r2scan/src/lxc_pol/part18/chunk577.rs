//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 577/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk577(t44: f64, t51: f64, t3190: f64, t552: f64, t551: f64, t3016: f64, t506: f64, t529: f64, t2999: f64, t3002: f64, t472: f64, t99: f64, t101: f64, t3007: f64, t3010: f64, t476: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t3191 = t552 * t3190;
    let t3192 = t551 * t3191;
    let t3197 = t506 * t3016;
    let t3198 = t529 * t3197;
    let t3208 = piecewise3(t45, 0.0_f64, 10.0_f64 / 9.0_f64 * t472 * t2999 + 5.0_f64 / 3.0_f64 * t99 * t3002);
    let t3214 = piecewise3(t52, 0.0_f64, 10.0_f64 / 9.0_f64 * t476 * t3007 + 5.0_f64 / 3.0_f64 * t101 * t3010);
    (t3191, t3192, t3197, t3198, t3208, t3214)
}
