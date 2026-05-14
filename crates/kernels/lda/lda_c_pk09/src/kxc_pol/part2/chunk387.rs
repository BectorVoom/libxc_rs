//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 387/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk387<F: Float>(t2040: F, t2042: F, t1947: F, t471: F, t305: F, t450: F, t1819: F, t462: F, t1672: F, t463: F, t1754: F, t1765: F, t1684: F, t1735: F, t1732: F, t1738: F, t1762: F, t1769: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2044 = t2040 * t2042 / 6.0;
    let t2045 = t471 * t1947;
    let t2047 = t2045 * t2042 / 6.0;
    let t2052 = t450 * t305;
    let t2053 = t1819 * t2052;
    let t2056 = t462 * t1947;
    let t2058 = t2056 * t2042 / 6.0;
    let t2060 = t463 * t1672 / 18.0;
    let t2061 = 1.5323028051206833 * t1754;
    let t2063 = 0.5107676017068944 * t1765;
    let t2065 = 0.3056501876701794 * t1684;
    let t2067 = 0.1018833958900598 * t1735;
    let t2069 = t2061 - 1.5323028051206833 * t1762 + t2063 + 1.5323028051206833 * t1769 + t2065 - 0.3056501876701794 * t1732 + t2067 + 0.3056501876701794 * t1738;
    (t2044, t2045, t2047, t2052, t2053, t2056, t2058, t2060, t2061, t2063, t2065, t2067, t2069)
}
