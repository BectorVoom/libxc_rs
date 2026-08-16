//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 816/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk816(t51: f64, t1368: f64, t3010: f64, t8571: f64, t1217: f64, t2474: f64, t419: f64, t53: f64, t8576: f64, t8575: f64, t60: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t8581 = t1368 * t3010;
    let t8584 = -t8571;
    let t8588 = piecewise3(t52, 0.0_f64, -8.0_f64 / 27.0_f64 * t8576 * t419 - 16.0_f64 / 9.0_f64 * t2474 * t1217 + 4.0_f64 / 9.0_f64 * t8581 * t419 + 4.0_f64 / 3.0_f64 * t53 * t8584);
    let t8589 = t8575 + t8588;
    let t8590 = t8589 * t60;
    (t8584, t8589, t8590)
}
