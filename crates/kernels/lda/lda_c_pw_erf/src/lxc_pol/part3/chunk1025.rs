//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1025/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1025<F: Float>(t2146: F, t3873: F, t3841: F, t1446: F, t4834: F, t5234: F, t5238: F, t4804: F, t5276: F, t3794: F, t3476: F, t5146: F) -> (F, F, F, F, F, F, F, F) {
    let t12012 = F::new(8.0) / F::new(15.0) * t2146 * t3873;
    let t12014 = F::new(8.0) / F::new(15.0) * t2146 * t3841;
    let t12015 = t1446 * t4834;
    let t12016 = F::new(16.0) / F::new(45.0) * t12015;
    let t12017 = t1446 * t5234;
    let t12018 = F::new(32.0) / F::new(45.0) * t12017;
    let t12019 = t1446 * t5238;
    let t12020 = F::new(16.0) / F::new(27.0) * t12019;
    let t12022 = F::new(8.0) / F::new(15.0) * t4804 * t5276;
    let t12024 = F::new(8.0) / F::new(15.0) * t3794 * t5276;
    let t12025 = t5146 * t3476;
    (t12012, t12014, t12016, t12018, t12020, t12022, t12024, t12025)
}
