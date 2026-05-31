//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 756/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk756<F: Float>(t34: F, t581: F, t593: F, t4892: F, t1318: F, t1336: F, t2146: F, t1124: F, t573: F, t2152: F, t571: F, t1446: F, t2143: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4893 = t581 * t34;
    let t4894 = t4893 * t593;
    let t4895 = t4892 * t4894;
    let t4897 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t4895;
    let t4899 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2146 * t1336;
    let t4900 = t1124 * t573;
    let t4901 = t4900 * t2152;
    let t4902 = t571 * t4901;
    let t4903 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4902;
    let t4905 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t1446 * t2143;
    (t4893, t4894, t4895, t4897, t4899, t4900, t4901, t4903, t4905)
}
