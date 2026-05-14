//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1027/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1027<F: Float>(t2067: F, t2425: F, t184: F, t784: F, t793: F, t2131: F, t493: F, t514: F, t7798: F, t15761: F, t786: F, t2123: F, t2402: F, t21423: F, t21426: F, t21427: F, t21428: F, t21430: F, t21431: F, t21432: F, t21436: F) -> (F, F, F, F, F, F) {
    let t21438 = 2.0 / 5.0 * t2425 * t2067;
    let t21440 = t784 * t793 * t184;
    let t21442 = 8.0 / 5.0 * t21440 * t2131;
    let t21444 = t493 * t514 * t7798;
    let t21445 = 8.0 / 45.0 * t21444;
    let t21447 = 4.0 / 5.0 * t15761 * t786;
    let t21448 = t2402 * t2123;
    let t21449 = 8.0 / 15.0 * t21448;
    let t21450 = t21423 + t21426 - t21427 + t21428 - t21430 - t21431 - t21432 + t21436 - t21438 + t21442 + t21445 + t21447 - t21449;
    (t21438, t21442, t21445, t21447, t21449, t21450)
}
