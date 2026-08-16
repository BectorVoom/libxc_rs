//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1172/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1172(t21079: f64, t2501: f64, t5220: f64, t17909: f64, t6836: f64, t802: f64, t6843: f64, t831: f64, t14482: f64, t14484: f64, t21068: f64, t21069: f64, t21071: f64, t21074: f64, t21078: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21080 = 2.0_f64 / 45.0_f64 * t21079;
    let t21081 = t5220 * t2501;
    let t21082 = 4.0_f64 / 45.0_f64 * t21081;
    let t21083 = 4.0_f64 / 135.0_f64 * t17909;
    let t21085 = t802 * t6836;
    let t21086 = t21085 / 15.0_f64;
    let t21087 = t831 * t6843;
    let t21088 = t21087 / 15.0_f64;
    let t21089 = -t21068 + t21069 + t21071 + t21074 + t21078 - t21080 - t21082 + t21083 + t14482 + 4.0_f64 * t14484 + t21086 + t21088;
    (t21080, t21082, t21083, t21086, t21088, t21089)
}
