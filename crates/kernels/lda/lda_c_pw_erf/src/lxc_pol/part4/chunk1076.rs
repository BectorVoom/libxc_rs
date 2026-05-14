//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1076/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1076<F: Float>(t15672: F, t2011: F, t5334: F, t2146: F, t4838: F, t4843: F, t2014: F, t15634: F, t15639: F, t15644: F, t15646: F, t15648: F, t15650: F, t15654: F, t15658: F, t15660: F, t15662: F, t15666: F, t15671: F) -> (F, F, F, F, F, F) {
    let t15673 = 32.0 / 81.0 * t15672;
    let t15675 = 16.0 / 45.0 * t5334 * t2011;
    let t15677 = 8.0 / 45.0 * t2146 * t4838;
    let t15679 = 32.0 / 45.0 * t2146 * t4843;
    let t15681 = 32.0 / 45.0 * t5334 * t2014;
    let t15682 = t15634 - t15639 + t15644 + t15646 - t15648 + t15650 + t15654 + t15658 + t15660 + t15662 + t15666 + t15671 + t15673 - t15675 - t15677 + t15679 - t15681;
    (t15673, t15675, t15677, t15679, t15681, t15682)
}
