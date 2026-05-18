//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 410/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk410<F: Float>(t153: F, t1540: F, t137: F, t132: F, t432: F, t436: F, t109: F, t136: F) -> (F, F, F, F, F, F) {
    let t1541 = t1540 * t153;
    let t1542 = t137 * t1541;
    let t1544 = t132 * t1542 / F::new(30.0);
    let t1545 = t432 * t436;
    let t1546 = F::new(2.0) / F::new(45.0) * t1545;
    let t1547 = t109 * t136;
    (t1541, t1542, t1544, t1545, t1546, t1547)
}
