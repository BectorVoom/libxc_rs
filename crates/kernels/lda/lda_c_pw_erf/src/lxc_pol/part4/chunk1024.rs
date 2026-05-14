//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1024/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1024<F: Float>(t1446: F, t5397: F, t2146: F, t3763: F, t3900: F, t4763: F, t4588: F, t518: F, t3899: F, t4929: F, t571: F, t3748: F, t1318: F, t2157: F, t9432: F, t1472: F, t5342: F) -> (F, F, F, F, F, F, F, F) {
    let t13470 = t1446 * t5397;
    let t13478 = t2146 * t3763;
    let t13480 = t4763 * t3900;
    let t13487 = t4588 * t518;
    let t13493 = t571 * t3899 * t4929;
    let t13495 = t2146 * t3748;
    let t13507 = t1318 * t9432 * t2157;
    let t13511 = t1472 * t5342;
    (t13470, t13478, t13480, t13487, t13493, t13495, t13507, t13511)
}
