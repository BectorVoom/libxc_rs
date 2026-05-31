//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1172/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1172<F: Float>(t21079: F, t2501: F, t5220: F, t17909: F, t6836: F, t802: F, t6843: F, t831: F, t14482: F, t14484: F, t21068: F, t21069: F, t21071: F, t21074: F, t21078: F) -> (F, F, F, F, F, F) {
    let t21080 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t21079;
    let t21081 = t5220 * t2501;
    let t21082 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t21081;
    let t21083 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t17909;
    let t21085 = t802 * t6836;
    let t21086 = t21085 / F::cast_from(15.0_f64);
    let t21087 = t831 * t6843;
    let t21088 = t21087 / F::cast_from(15.0_f64);
    let t21089 = -t21068 + t21069 + t21071 + t21074 + t21078 - t21080 - t21082 + t21083 + t14482 + F::cast_from(4.0_f64) * t14484 + t21086 + t21088;
    (t21080, t21082, t21083, t21086, t21088, t21089)
}
