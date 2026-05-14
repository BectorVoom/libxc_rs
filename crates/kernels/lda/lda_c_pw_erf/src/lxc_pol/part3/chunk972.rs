//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 972/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk972<F: Float>(t12929: F, t3974: F, t4522: F, t10392: F, t4508: F, t559: F, t2137: F, t5041: F, t5045: F, t2120: F, t3466: F, t10039: F, t10164: F, t10167: F, t4647: F, t515: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12937 = 16.0 / 9.0 * t3974 * t4522 * t12929;
    let t12941 = 32.0 / 15.0 * t3974 * t4508 * t559 * t10392;
    let t12942 = t5041 * t2137;
    let t12943 = 8.0 / 15.0 * t12942;
    let t12944 = t5045 * t2137;
    let t12945 = 16.0 / 15.0 * t12944;
    let t12947 = 4.0 / 15.0 * t2120 * t3466;
    let t12948 = 8.0 / 15.0 * t10039;
    let t12949 = 16.0 / 45.0 * t10164;
    let t12950 = 32.0 / 405.0 * t10167;
    let t12951 = t4647 * t515;
    (t12937, t12941, t12943, t12945, t12947, t12948, t12949, t12950, t12951)
}
