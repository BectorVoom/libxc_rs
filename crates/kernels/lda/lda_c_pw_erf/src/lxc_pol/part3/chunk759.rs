//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 759/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk759<F: Float>(t1403: F, t3667: F, t833: F, t1466: F, t571: F, t2157: F, t3899: F, t1318: F, t1392: F, t3675: F, t806: F, t1440: F) -> (F, F, F, F, F, F, F) {
    let t4929 = t3667 * t833 * t1403;
    let t4930 = t1466 * t4929;
    let t4932 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t571 * t4930;
    let t4933 = t3899 * t2157;
    let t4935 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1318 * t4933;
    let t4937 = t3675 * t806 * t1392;
    let t4938 = t1440 * t4937;
    (t4929, t4930, t4932, t4933, t4935, t4937, t4938)
}
