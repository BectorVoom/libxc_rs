//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 567/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk567<F: Float>(t1419: F, t656: F, t245: F, t646: F, t1426: F, t645: F, t1433: F, t1: F, t1578: F, t119: F, t1432: F, t247: F, t24: F, t1953: F, t2061: F, t248: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3910 = t1419 * t656;
    let t3912 = t245 * t646;
    let t3915 = t245 * t1426;
    let t3917 = 2e-21 * t645 * t3915;
    let t3919 = 2.0 / 3.0 * t1433 * t656;
    let t3920 = t1578 * t1;
    let t3921 = t119 * t646;
    let t3923 = 0.001515438175925926 * t3920 * t3921;
    let t3926 = t247 * t1432;
    let t3927 = t24 * t3926;
    let t3929 = 0.18233333333333332 * t645 * t3927;
    let t3932 = 0.1005925925925926 * t1953 - 0.5007407407407407 * t2061;
    let t3933 = t248 * t3932;
    (t3910, t3912, t3915, t3917, t3919, t3920, t3921, t3923, t3926, t3927, t3929, t3932, t3933)
}
