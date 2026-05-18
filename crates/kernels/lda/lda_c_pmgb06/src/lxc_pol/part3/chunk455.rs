//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 455/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk455<F: Float>(t12: F, t176: F, t1835: F, t166: F, t161: F, t337: F, t764: F, t1: F, t395: F, t44: F, t131: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t1836 = t1835 * t176;
    let t1837 = t166 * t1836;
    let t1839 = t161 * t1837 / F::new(30.0);
    let t1840 = t337 * t764;
    let t1842 = t12 * t1;
    let t1846 = piecewise3::<f64>(t13, F::new(0.0), -F::new(4.0) * t1842 * t395 + F::new(2.0) * t1840);
    let t1847 = t1846 * t44;
    let t1848 = t1847 * t131;
    (t1836, t1837, t1839, t1840, t1842, t1847, t1848)
}
