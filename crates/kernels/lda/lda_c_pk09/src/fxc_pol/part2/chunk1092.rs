//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1092/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1092<F: Float>(t11679: F, t309: F, t1950: F, t11101: F, t501: F, t1819: F, t2758: F, t1930: F, t1800: F, t11466: F, t1847: F, t497: F) -> (F, F, F, F, F, F) {
    let t11995 = t11679 * t309;
    let t11996 = t11995 * t1950;
    let t12000 = t501 * t11101;
    let t12003 = t1819 * t2758;
    let t12006 = t1930 * t11101;
    let t12007 = t12006 * t1800;
    let t12009 = t1847 * t11466;
    let t12011 = t497 * t11101;
    (t11996, t12000, t12003, t12007, t12009, t12011)
}
