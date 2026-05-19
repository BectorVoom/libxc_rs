//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1351/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1351<F: Float>(t10319: F, t10321: F, t10335: F, t10339: F, t161: F, t489: F, t6832: F, t10743: F, t10746: F, t10751: F, t10754: F, t10757: F, t10760: F, t10764: F, t10769: F, t10770: F, t10773: F) -> (F, F, F, F, F, F) {
    let t17766 = F::new(4.0) / F::new(405.0) * t10319;
    let t17767 = F::new(16.0) / F::new(1215.0) * t10321;
    let t17768 = F::new(16.0) / F::new(1215.0) * t10335;
    let t17769 = F::new(4.0) / F::new(405.0) * t10339;
    let t17771 = t161 * t489 * t6832;
    let t17772 = F::new(2.0) / F::new(45.0) * t17771;
    let t17779 = -t17766 + t17767 + t17768 + t17769 + t17772 + F::cast_from(0.4328416544945937_f64) * t10743 + t10746 + F::cast_from(0.21642082724729686_f64) * t10751 + F::cast_from(0.011181742741110338_f64) * t10754 + F::cast_from(0.6492624817418906_f64) * t10757 + F::cast_from(0.06709045644666203_f64) * t10760 + t10764 + t10769 - F::cast_from(0.19237406866426388_f64) * t10770 - t10773;
    (t17766, t17767, t17768, t17769, t17772, t17779)
}
