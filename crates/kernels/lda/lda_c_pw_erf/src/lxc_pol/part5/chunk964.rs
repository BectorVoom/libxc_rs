//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 964/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk964<F: Float>(t13048: F, t10654: F, t1949: F, t571: F, t1401: F, t2151: F, t219: F, t4900: F, t3704: F, t3973: F, t1333: F, t4507: F) -> (F, F, F, F, F, F) {
    let t13049 = F::new(8.0) / F::new(135.0) * t13048;
    let t13051 = t571 * t10654 * t1949;
    let t13052 = F::new(16.0) / F::new(135.0) * t13051;
    let t13060 = t2151 * t1401;
    let t13080 = t4900 * t219;
    let t13115 = t3973 * t3704;
    let t13122 = t4507 * t1333;
    (t13049, t13052, t13060, t13080, t13115, t13122)
}
