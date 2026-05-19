//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 406/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk406<F: Float>(t2040: F, t2042: F, t1947: F, t471: F, t305: F, t450: F, t1819: F, t462: F, t1672: F, t463: F, t1754: F, t1765: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2044 = t2040 * t2042 / F::new(6.0);
    let t2045 = t471 * t1947;
    let t2047 = t2045 * t2042 / F::new(6.0);
    let t2052 = t450 * t305;
    let t2053 = t1819 * t2052;
    let t2056 = t462 * t1947;
    let t2058 = t2056 * t2042 / F::new(6.0);
    let t2060 = t463 * t1672 / F::new(18.0);
    let t2061 = F::cast_from(1.5323028051206833_f64) * t1754;
    let t2063 = F::cast_from(0.5107676017068944_f64) * t1765;
    (t2044, t2045, t2047, t2052, t2053, t2056, t2058, t2060, t2061, t2063)
}
