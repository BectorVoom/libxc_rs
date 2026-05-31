//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 390/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk390<F: Float>(t5: F, t153: F, t1872: F, t137: F, t132: F, t460: F, t802: F, t332: F, t760: F, t1: F, t395: F, t44: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t1873 = t1872 * t153;
    let t1874 = t137 * t1873;
    let t1876 = t132 * t1874 / F::cast_from(30.0_f64);
    let t1878 = t802 * t460 / F::cast_from(30.0_f64);
    let t1879 = t332 * t760;
    let t1881 = t5 * t1;
    let t1885 = piecewise3::<F>(t6, F::cast_from(0.0_f64), F::cast_from(4.0_f64) * t1881 * t395 + F::cast_from(2.0_f64) * t1879);
    let t1886 = t1885 * t44;
    (t1873, t1874, t1876, t1878, t1886)
}
