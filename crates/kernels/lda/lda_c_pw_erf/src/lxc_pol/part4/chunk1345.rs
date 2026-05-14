//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1345/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1345<F: Float>(t632: F, t7032: F, t14941: F, t14943: F, t14945: F, t14947: F, t14950: F, t14954: F, t14956: F, t14958: F, t14960: F, t18737: F, t242: F, t1143: F, t2379: F, t6138: F) -> (F, F, F) {
    let t19385 = t7032 * t632;
    let t19387 = -0.21223777183119583 * t14941 - 1.0051538464260528 * t14943 + 0.1675256410710088 * t14945 + 0.6701025642840353 * t14947 + 0.5025769232130264 * t14950 - 0.1675256410710088 * t14954 - 0.3350512821420176 * t14956 - 0.1675256410710088 * t14958 - 0.6701025642840353 * t14960 - 0.0837628205355044 * t18737 * t242 - 0.1675256410710088 * t19385;
    let t19388 = t2379 * t1143;
    let t19397 = t6138 * t242;
    (t19387, t19388, t19397)
}
