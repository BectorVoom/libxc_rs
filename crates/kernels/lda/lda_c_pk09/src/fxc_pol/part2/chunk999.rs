//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 999/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk999<F: Float>(t10791: F, t1470: F, t1214: F, t2520: F, t93: F, t2143: F, t3677: F, t623: F, t10020: F, t1487: F, t1406: F, t9836: F) -> (F, F, F, F, F) {
    let t10792 = t1470 * t10791;
    let t10794 = t2520 * t1214;
    let t10795 = t93 * t10794;
    let t10798 = t3677 * t2143;
    let t10799 = t10798 * t623;
    let t10800 = t93 * t10799;
    let t10803 = t1487 * t10020;
    let t10808 = t1406 * t9836;
    (t10792, t10795, t10800, t10803, t10808)
}
