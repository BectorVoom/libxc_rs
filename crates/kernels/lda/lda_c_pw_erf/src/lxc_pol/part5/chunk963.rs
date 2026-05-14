//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 963/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk963<F: Float>(t102: F, t120: F, t20283: F, t19571: F, t19574: F, t19577: F, t127: F, t19580: F, t19584: F, t19590: F, t19593: F, t19604: F, t19614: F, t436: F, t19645: F, t19647: F) -> (F, F, F, F, F, F, F) {
    let t20324 = 2.923025 * t102 * t120 * t20283;
    let t20328 = 1.9486833333333333 * t19571;
    let t20329 = 0.9743416666666667 * t19574;
    let t20330 = 1.4615125 * t19577;
    let t20337 = -t20324 - 1.46904 * t127 * t436 * t20283 - t20328 + t20329 + t20330 + 44.0712 * t19580 - 17.62848 * t19584 + 6.0 * t19590 - 3.0 * t19593 - 3.0 / 2.0 * t19604 - 8.81424 * t19614;
    let t20340 = 8.769075 * t19645;
    let t20341 = 5.84605 * t19647;
    (t20324, t20328, t20329, t20330, t20337, t20340, t20341)
}
