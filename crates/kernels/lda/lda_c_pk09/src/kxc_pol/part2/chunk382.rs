//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 382/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk382<F: Float>(t1729: F, t497: F, t489: F, t1941: F, t337: F, t430: F, t1805: F, t476: F, t1468: F, t10: F, t1838: F, t1947: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1979 = t497 * t1729;
    let t1982 = t489 * t1729;
    let t1985 = t1941 * t337;
    let t1986 = t1985 * t430;
    let t1989 = t476 * t1805;
    let t1991 = t1468 * t430;
    let t1992 = t1991 * t10;
    let t1993 = t1992 * t1838;
    let t1994 = 5.40024514194619 * t1993;
    let t1995 = t1947 * t10;
    (t1979, t1982, t1985, t1986, t1989, t1991, t1992, t1993, t1994, t1995)
}
