//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1013/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1013(t328: f64, t8019: f64, t11568: f64, t11569: f64, t11601: f64, t11604: f64, t11609: f64, t11611: f64, t11615: f64, t11617: f64, t11624: f64, t11629: f64, t11633: f64, t1316: f64, t1317: f64, t14786: f64, t14789: f64, t14852: f64, t346: f64, t6021: f64, t6028: f64, t7354: f64) -> (f64, f64) {
    let t19076 = t8019 * t328;
    let t19092 = -0.0008717022455366076_f64 * t14786 - 0.0017434044910732151_f64 * t14789 - t11568 - 0.47896936041018434_f64 * t11569 + 3.0_f64 * t1316 * t7354 * t1317 - 2.0_f64 * t346 * t6021 * t6028 - t11601 - 0.03592270203076383_f64 * t11604 - t11609 - 0.0001639671923854359_f64 * t11611 - 1.370765728342244e-05_f64 * t11615 + 0.585406996056892_f64 * t11617 - 2.7743564462147594_f64 * t11624 + t11629 + t11633 - 0.16213771438917426_f64 * t14852;
    (t19076, t19092)
}
