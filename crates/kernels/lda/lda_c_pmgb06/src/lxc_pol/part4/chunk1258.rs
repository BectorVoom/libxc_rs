//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1258/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1258<F: Float>(t5120: F, t802: F, t12981: F, t6633: F, t1594: F, t3032: F, t443: F, t5077: F, t6637: F, t13007: F, t6562: F, t1602: F, t3458: F, t497: F, t5068: F, t6560: F) -> (F, F, F, F, F) {
    let t16541 = F::new(2.0) / F::new(15.0) * t802 * t5120;
    let t16542 = t12981 * t6633;
    let t16543 = F::new(8.0) / F::new(81.0) * t16542;
    let t16548 = F::new(4.0) / F::new(15.0) * t5077 * t3032 * t443 * t6637 * t1594;
    let t16549 = t13007 * t6562;
    let t16550 = F::new(16.0) / F::new(135.0) * t16549;
    let t16555 = F::new(4.0) / F::new(15.0) * t5068 * t3458 * t497 * t6560 * t1602;
    (t16541, t16543, t16548, t16550, t16555)
}
