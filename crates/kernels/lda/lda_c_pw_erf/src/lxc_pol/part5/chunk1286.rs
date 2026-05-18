//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1286/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1286<F: Float>(t173: F, t184: F, t199: F, t23004: F, t23025: F, t18681: F, t826: F, t15144: F, t15146: F, t15147: F, t19318: F, t19320: F, t22967: F, t22971: F, t22975: F, t22978: F, t22981: F, t22983: F) -> (F, F, F) {
    let t23030 = F::new(2.0) / F::new(15.0) * t173 * (t23004 + t23025) * t184 * t199;
    let t23032 = F::new(4.0) / F::new(15.0) * t18681 * t826;
    let t23033 = t22967 + t22971 - t22975 + t22978 + t22981 + t15144 + t15146 + F::new(2.0) * t15147 + t22983 + F::new(2.0) / F::new(3.0) * t19318 + F::new(4.0) / F::new(3.0) * t19320 + t23030 + t23032;
    (t23030, t23032, t23033)
}
