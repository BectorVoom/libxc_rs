//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1237/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1237<F: Float>(t486: F, t6596: F, t4948: F, t831: F, t1499: F, t2625: F, t6616: F, t12274: F, t12276: F, t12278: F, t12281: F, t132: F, t1547: F, t2583: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16293 = t486 * t6596 / F::cast_from(15.0_f64);
    let t16294 = t831 * t4948;
    let t16295 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t16294;
    let t16297 = t1499 * t2625 / F::cast_from(30.0_f64);
    let t16298 = t486 * t6616;
    let t16299 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t16298;
    let t16300 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t12274;
    let t16301 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t12276;
    let t16302 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t12278;
    let t16303 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t12281;
    let t16305 = t132 * t1547 * t2583;
    (t16293, t16295, t16297, t16299, t16300, t16301, t16302, t16303, t16305)
}
