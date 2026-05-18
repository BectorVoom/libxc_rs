//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 593/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk593<F: Float>(t24: F, t3926: F, t645: F, t1953: F, t2061: F, t248: F, t256: F, t635: F, t646: F, t1432: F, t639: F, t1423: F, t1427: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3927 = t24 * t3926;
    let t3929 = F::new(0.18233333333333332) * t645 * t3927;
    let t3932 = F::new(0.1005925925925926) * t1953 - F::new(0.5007407407407407) * t2061;
    let t3933 = t248 * t3932;
    let t3935 = t3933 * t256 / F::new(3.0);
    let t3936 = t635 * t646;
    let t3938 = F::new(0.013506172839506173) * t645 * t3936;
    let t3949 = t639 * t1432;
    let t3950 = t3949 * t256;
    let t3951 = t1423 * t1427;
    (t3927, t3929, t3932, t3933, t3935, t3936, t3938, t3949, t3950, t3951)
}
