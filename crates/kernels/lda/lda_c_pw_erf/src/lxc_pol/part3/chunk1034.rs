//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1034/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1034<F: Float>(t10614: F, t10617: F, t10620: F, t1518: F, t185: F, t2099: F, t3671: F, t822: F, t3846: F, t3965: F, t4479: F, t3850: F, t4500: F, t784: F, t3403: F, t3412: F, t6762: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14000 = 8.0 / 27.0 * t10614;
    let t14001 = 16.0 / 15.0 * t10617;
    let t14002 = 16.0 / 135.0 * t10620;
    let t14004 = t185 * t1518 * t2099;
    let t14005 = 4.0 / 45.0 * t14004;
    let t14007 = 4.0 / 5.0 * t822 * t3671;
    let t14010 = 8.0 / 15.0 * t3965 * t4479 * t3846;
    let t14013 = 8.0 / 15.0 * t3965 * t4479 * t3850;
    let t14014 = t4500 * t784;
    let t14017 = 8.0 / 9.0 * t3965 * t14014 * t3403;
    let t14020 = 16.0 / 15.0 * t3965 * t6762 * t3412;
    (t14000, t14001, t14002, t14005, t14007, t14010, t14013, t14017, t14020)
}
