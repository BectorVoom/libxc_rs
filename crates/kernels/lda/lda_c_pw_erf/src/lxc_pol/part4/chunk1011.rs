//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1011/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1011<F: Float>(t5064: F, t518: F, t2146: F, t3864: F, t4059: F, t1124: F, t213: F, t1318: F, t4894: F, t4063: F, t2137: F, t5041: F, t5045: F, t4647: F, t515: F, t4489: F, t784: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12881 = t5064 * t518;
    let t12890 = t2146 * t3864;
    let t12908 = t2146 * t4059;
    let t12916 = t1124 * t213;
    let t12918 = t1318 * t12916 * t4894;
    let t12924 = t2146 * t4063;
    let t12942 = t5041 * t2137;
    let t12944 = t5045 * t2137;
    let t12951 = t4647 * t515;
    let t12956 = t4489 * t784;
    (t12881, t12890, t12908, t12916, t12918, t12924, t12942, t12944, t12951, t12956)
}
