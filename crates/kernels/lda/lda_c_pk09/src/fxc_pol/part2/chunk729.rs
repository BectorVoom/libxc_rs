//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 729/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk729<F: Float>(t1800: F, t7516: F, t1828: F, t6488: F, t1859: F, t6477: F, t502: F, t6601: F, t1872: F, t1947: F, t2042: F, t1877: F, t6196: F) -> (F, F, F, F, F, F) {
    let t7517 = t7516 * t1800;
    let t7522 = F::new(1.2536914064583544) * t1828 * t6488;
    let t7523 = t1859 * t6477;
    let t7526 = F::new(8.661855010924512) * t502 * t6601;
    let t7527 = t1872 * t1947;
    let t7528 = t7527 * t2042;
    let t7530 = t1877 * t6196;
    (t7517, t7522, t7523, t7526, t7528, t7530)
}
