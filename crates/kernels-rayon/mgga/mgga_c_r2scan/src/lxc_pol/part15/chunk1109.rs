//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1109/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1109(t3270: f64, t39311: f64, t3269: f64, t10634: f64, t11629: f64, t3262: f64, t1563: f64, t3582: f64, t3275: f64, t37299: f64, t37390: f64, t39276: f64, t39278: f64, t39282: f64, t39284: f64, t39289: f64, t39290: f64, t39295: f64, t39298: f64, t39303: f64, t39306: f64, t39309: f64) -> (f64, f64, f64, f64) {
    let t39312 = t3270 * t39311;
    let t39314 = t3269 * t39312 / 2.0_f64;
    let t39317 = 15.0_f64 / 8.0_f64 * t3262 * t11629 * t10634;
    let t39318 = t3582 * t1563;
    let t39321 = 585.0_f64 / 256.0_f64 * t3275 * t37299 * t39318;
    let t39322 = -t39276 + t39278 - t39282 - t39284 + 0.19211284388664477842e-2_f64 * t37390 - t39289 + 0.30487649791575028314e-3_f64 * t39290 - t39295 - t39298 - t39303 - t39306 + 0.15243824895787514157e-3_f64 * t39309 + t39314 + t39317 + t39321;
    (t39314, t39317, t39321, t39322)
}
