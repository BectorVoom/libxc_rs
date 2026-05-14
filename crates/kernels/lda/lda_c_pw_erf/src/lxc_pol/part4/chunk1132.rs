//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1132/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1132<F: Float>(t12118: F, t6720: F, t3965: F, t4637: F, t6762: F, t4615: F, t6766: F, t4624: F, t12030: F, t4610: F, t784: F, t10011: F, t6759: F, t6763: F, t6767: F, t4479: F, t5243: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16633 = t12118 * t6720;
    let t16634 = 32.0 / 81.0 * t16633;
    let t16637 = 32.0 / 45.0 * t3965 * t6762 * t4637;
    let t16640 = 32.0 / 9.0 * t3965 * t6766 * t4615;
    let t16643 = 16.0 / 27.0 * t3965 * t6766 * t4624;
    let t16647 = 128.0 / 81.0 * t3965 * t12030 * t784 * t4610;
    let t16648 = t10011 * t6759;
    let t16649 = 64.0 / 135.0 * t16648;
    let t16650 = t10011 * t6763;
    let t16651 = 128.0 / 135.0 * t16650;
    let t16652 = t10011 * t6767;
    let t16653 = 64.0 / 81.0 * t16652;
    let t16656 = 16.0 / 45.0 * t3965 * t4479 * t5243;
    (t16634, t16637, t16640, t16643, t16647, t16649, t16651, t16653, t16656)
}
