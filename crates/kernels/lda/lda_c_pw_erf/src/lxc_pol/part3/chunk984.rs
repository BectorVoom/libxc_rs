//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 984/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk984<F: Float>(t2146: F, t4052: F, t10488: F, t826: F, t2140: F, t3742: F, t2143: F, t3745: F, t1401: F, t1466: F, t5029: F, t571: F, t593: F, t3757: F, t13127: F, t13129: F, t13131: F, t13133: F, t13135: F, t13137: F, t13139: F) -> (F, F, F, F, F, F, F) {
    let t13141 = 32.0 / 81.0 * t2146 * t4052;
    let t13143 = 4.0 / 45.0 * t10488 * t826;
    let t13144 = t3742 * t2140;
    let t13145 = 16.0 / 45.0 * t13144;
    let t13146 = t3745 * t2143;
    let t13147 = 16.0 / 45.0 * t13146;
    let t13152 = 4.0 / 5.0 * t571 * t1466 * t1401 * t5029 * t593;
    let t13154 = 4.0 / 5.0 * t2146 * t3757;
    let t13155 = -t13127 - t13129 - t13131 + t13133 - t13135 - t13137 + t13139 + t13141 + t13143 + t13145 + t13147 + t13152 + t13154;
    (t13141, t13143, t13145, t13147, t13152, t13154, t13155)
}
