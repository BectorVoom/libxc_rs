//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1239/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1239(t27357: f64, t5426: f64, t98233: f64, t12185: f64, t1307: f64, t1650: f64, t27359: f64, t27369: f64, t27459: f64, t28439: f64, t3984: f64, t4001: f64, t7908: f64, t7909: f64, t94227: f64, t94287: f64, t94289: f64, t94626: f64, t98155: f64, t98205: f64, t98220: f64, t98226: f64, t98230: f64) -> (f64, f64) {
    let t98235 = t98233 * t5426 * t27357;
    let t98238 = 0.46336805555555555556e-3_f64 * t7908 * t3984 * t98205 * t1307 - 0.16489724537037037037e-3_f64 * t98155 * t27359 - 0.46336805555555555556e-3_f64 * t7908 * t12185 * t7909 * t1650 * t4001 + 0.46336805555555555556e-3_f64 * t27459 * t28439 - 0.46336805555555555556e-3_f64 * t7908 * t98220 - 0.61836467013888888888e-4_f64 * t27369 * t98220 + t98226 + 0.10297067901234567901e-3_f64 * t94287 - 0.15445601851851851852e-3_f64 * t94289 - 0.46336805555555555556e-3_f64 * t94626 * t98230 + 0.82448622685185185186e-4_f64 * t94227 * t98235;
    (t98235, t98238)
}
