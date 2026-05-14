//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 655/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk655<F: Float>(t497: F, t6287: F, t1831: F, t1800: F, t1827: F, t501: F, t1971: F, t309: F, t1876: F, t1828: F, t6488: F, t1859: F, t6477: F, t502: F, t6601: F, t1872: F, t1947: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7494 = t497 * t6287;
    let t7500 = t1831 * t6287;
    let t7501 = t7500 * t1800;
    let t7503 = t1827 * t6287;
    let t7504 = t7503 * t1800;
    let t7506 = t501 * t6287;
    let t7513 = t1971 * t309;
    let t7516 = t1876 * t6287;
    let t7517 = t7516 * t1800;
    let t7522 = 1.2536914064583544 * t1828 * t6488;
    let t7523 = t1859 * t6477;
    let t7526 = 8.661855010924512 * t502 * t6601;
    let t7527 = t1872 * t1947;
    (t7494, t7501, t7504, t7506, t7513, t7517, t7522, t7523, t7526, t7527)
}
