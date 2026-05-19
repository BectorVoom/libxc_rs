//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1464/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1464<F: Float>(t18751: F, t69: F, t18754: F, t18616: F, t18634: F, t18637: F, t18638: F, t18640: F, t18644: F, t18646: F, t18650: F, t18737: F, t18741: F, t2247: F, t8263: F, t8287: F, t8295: F, t8441: F) -> F {
    let t18829 = t69 * t18751;
    let t18831 = t69 * t18754;
    let t18835 = -F::cast_from(0.7663355555555555_f64) * t8441 - t18616 - F::new(82.76424) * t2247 * t18650 + t8263 + t18634 + t8287 - t8295 - t18637 - t18638 + t18640 + t18644 - t18646 - F::new(1.724255) * t69 * t18737 + F::cast_from(1.1495033333333333_f64) * t18829 + F::cast_from(2.2990066666666666_f64) * t18831 + F::new(10.34553) * t69 * t18741;
    t18835
}
