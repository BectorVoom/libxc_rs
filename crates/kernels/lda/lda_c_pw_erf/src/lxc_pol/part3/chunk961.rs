//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 961/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk961<F: Float>(t242: F, t4130: F, t1155: F, t632: F, t4137: F, t1198: F, t1426: F, t1159: F, t646: F, t695: F, t3926: F, t458: F) -> (F, F, F, F, F, F, F) {
    let t11007 = t4130 * t242;
    let t11010 = F::new(2.0103076928521055) * t1155 * t632;
    let t11012 = F::new(2.0103076928521055) * t4137 * t242;
    let t11020 = t1198 * t1426;
    let t11022 = t1159 * t646;
    let t11025 = F::new(0.26596355555555556) * t695 * t1426;
    let t11027 = F::new(0.19947266666666666) * t458 * t3926;
    (t11007, t11010, t11012, t11020, t11022, t11025, t11027)
}
