//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 884/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk884<F: Float>(t1629: F, t1966: F, t439: F, t5201: F, t224: F, t4753: F, t446: F, t1427: F, t5187: F, t1431: F, t1441: F, t1447: F, t5176: F, t1989: F, t3226: F, t1499: F, t2090: F) -> (F, F, F, F, F, F, F, F) {
    let t11820 = t439 * t1966 * t5201 * t1629 / 5.0;
    let t11821 = t4753 * t224;
    let t11823 = t11821 * t446 / 15.0;
    let t11825 = 2.0 / 15.0 * t5187 * t1427;
    let t11827 = t5187 * t1431 / 15.0;
    let t11829 = t5187 * t1441 / 9.0;
    let t11830 = t1447 * t5176;
    let t11831 = 4.0 / 15.0 * t11830;
    let t11832 = t3226 * t1989;
    let t11833 = 4.0 / 45.0 * t11832;
    let t11835 = t1499 * t2090 / 10.0;
    (t11820, t11823, t11825, t11827, t11829, t11831, t11833, t11835)
}
