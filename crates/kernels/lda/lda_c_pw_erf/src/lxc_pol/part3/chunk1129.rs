//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1129/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1129<F: Float>(t13211: F, t4804: F, t5414: F, t3794: F, t3476: F, t784: F, t1325: F, t1991: F, t2954: F, t5418: F, t1976: F, t4829: F, t945: F) -> (F, F, F, F, F, F, F) {
    let t13212 = F::new(32.0) / F::new(45.0) * t13211;
    let t13214 = F::new(16.0) / F::new(15.0) * t4804 * t5414;
    let t13216 = F::new(16.0) / F::new(15.0) * t3794 * t5414;
    let t13217 = t784 * t3476;
    let t13221 = F::new(16.0) / F::new(9.0) * t1325 * t1991 * t13217 * t2954;
    let t13223 = F::new(16.0) / F::new(15.0) * t4804 * t5418;
    let t13225 = F::new(16.0) / F::new(15.0) * t3794 * t5418;
    let t13229 = F::new(8.0) / F::new(15.0) * t1325 * t4829 * t1976 * t945;
    (t13212, t13214, t13216, t13221, t13223, t13225, t13229)
}
