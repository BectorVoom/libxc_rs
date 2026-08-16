//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 621/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk621(t3176: f64, t3322: f64, t393: f64, t1138: f64, t1141: f64, t1203: f64, t1140: f64, t392: f64, t364: f64, t1171: f64, t1175: f64, t1170: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3323 = t3176 + t3322;
    let t3324 = t3323 * t393;
    let t3325 = t1138 * t1141;
    let t3327 = 2.0_f64 * t3325 * t1203;
    let t3329 = 1.0_f64 / t1140 / t392;
    let t3330 = t364 * t3329;
    let t3331 = t1203 * t1203;
    let t3333 = 2.0_f64 * t3330 * t3331;
    let t3334 = t1175 * t1171;
    let t3335 = t1170 * t3334;
    (t3323, t3324, t3325, t3327, t3329, t3330, t3331, t3333, t3334, t3335)
}
