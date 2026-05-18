//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 390/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk390<F: Float>(t514: F, t633: F, t1905: F, t1754: F, t1765: F, t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F, t513: F) -> (F, F, F, F, F, F, F, F) {
    let t1906 = t514 * t633;
    let t1907 = t1905 * t1906;
    let t1910 = F::new(4.0) * t1754;
    let t1912 = F::new(1.3333333333333333) * t1765;
    let t1914 = F::new(0.821419393556371) * t1684;
    let t1916 = F::new(0.2738064645187903) * t1735;
    let t1918 = t1910 - F::new(4.0) * t1762 + t1912 + F::new(4.0) * t1769 + t1914 - F::new(0.821419393556371) * t1732 + t1916 + F::new(0.821419393556371) * t1738;
    let t1919 = F::new(1.0) / t513;
    (t1906, t1907, t1910, t1912, t1914, t1916, t1918, t1919)
}
