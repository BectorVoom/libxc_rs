//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1281/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1281<F: Float>(t4941: F, t831: F, t13094: F, t4803: F, t132: F, t443: F, t4828: F, t814: F, t1420: F, t6245: F, t12063: F, t439: F, t805: F) -> (F, F, F, F, F, F) {
    let t16840 = F::new(2.0) / F::new(15.0) * t831 * t4941;
    let t16841 = F::new(4.0) / F::new(45.0) * t13094;
    let t16843 = F::new(2.0) / F::new(15.0) * t831 * t4803;
    let t16847 = F::new(4.0) / F::new(45.0) * t132 * t4828 * t814 * t443;
    let t16849 = F::new(2.0) / F::new(45.0) * t1420 * t6245;
    let t16852 = F::new(2.0) / F::new(45.0) * t439 * t12063 * t805;
    (t16840, t16841, t16843, t16847, t16849, t16852)
}
