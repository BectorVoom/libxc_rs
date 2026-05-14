//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 821/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk821<F: Float>(t1348: F, t9856: F, t1388: F, t2674: F, t747: F, t2520: F, t1481: F, t1490: F, t9836: F, t1435: F, t2637: F, t131: F, t2640: F, t1215: F, t2625: F, t372: F, t7766: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9857 = t1348 * t9856;
    let t9860 = t1388 * t747 * t2674;
    let t9862 = t747 * t2520;
    let t9863 = t1481 * t9862;
    let t9865 = t1490 * t9836;
    let t9867 = t2637 * t1435;
    let t9869 = t131 * t2640;
    let t9870 = t1348 * t9869;
    let t9874 = t2625 * t1215;
    let t9877 = t372 * t7766;
    (t9857, t9860, t9862, t9863, t9865, t9867, t9870, t9874, t9877)
}
