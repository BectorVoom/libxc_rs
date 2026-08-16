//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 725/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk725<F: Float>(t1672: F, t1965: F, t1968: F, t1959: F, t1962: F, t546: F, t6601: F, t1832: F, t6196: F, t1828: F, t1798: F, t1897: F, t1947: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7411 = t1965 * t1672;
    let t7413 = t1968 * t1672;
    let t7415 = t1959 * t1672;
    let t7418 = t1962 * t1672;
    let t7421 = F::cast_from(1.0788960867530346_f64) * t546 * t6601;
    let t7422 = t1832 * t6196;
    let t7426 = t1828 * t6196;
    let t7430 = t1798 * t6196;
    let t7432 = t1897 * t1947;
    (t7411, t7413, t7415, t7418, t7421, t7422, t7426, t7430, t7432)
}
