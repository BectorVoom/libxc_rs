//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 934/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk934<F: Float>(t12831: F, t1600: F, t1835: F, t1912: F, t3223: F, t1916: F, t1920: F, t1847: F, t607: F, t12514: F, t1461: F, t5065: F) -> (F, F, F, F, F, F, F) {
    let t12832 = t12831 / F::cast_from(45.0_f64);
    let t12840 = t1835 * t1600;
    let t12868 = t3223 * t1912;
    let t12869 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t12868;
    let t12870 = t3223 * t1916;
    let t12871 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t12870;
    let t12878 = t3223 * t1920;
    let t12879 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t12878;
    let t12912 = t1847 * t607;
    let t12981 = t5065 * t12514 * t1461;
    (t12832, t12840, t12869, t12871, t12879, t12912, t12981)
}
