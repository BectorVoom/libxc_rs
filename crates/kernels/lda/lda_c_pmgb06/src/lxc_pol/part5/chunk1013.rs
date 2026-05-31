//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1013/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1013<F: Float>(t328: F, t8019: F, t11568: F, t11569: F, t11601: F, t11604: F, t11609: F, t11611: F, t11615: F, t11617: F, t11624: F, t11629: F, t11633: F, t1316: F, t1317: F, t14786: F, t14789: F, t14852: F, t346: F, t6021: F, t6028: F, t7354: F) -> (F, F) {
    let t19076 = t8019 * t328;
    let t19092 = -F::cast_from(0.0008717022455366076_f64) * t14786 - F::cast_from(0.0017434044910732151_f64) * t14789 - t11568 - F::cast_from(0.47896936041018434_f64) * t11569 + F::cast_from(3.0_f64) * t1316 * t7354 * t1317 - F::cast_from(2.0_f64) * t346 * t6021 * t6028 - t11601 - F::cast_from(0.03592270203076383_f64) * t11604 - t11609 - F::cast_from(0.0001639671923854359_f64) * t11611 - F::cast_from(1.370765728342244e-05_f64) * t11615 + F::cast_from(0.585406996056892_f64) * t11617 - F::cast_from(2.7743564462147594_f64) * t11624 + t11629 + t11633 - F::cast_from(0.16213771438917426_f64) * t14852;
    (t19076, t19092)
}
