//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1112/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1112<F: Float>(t4753: F, t4895: F, t3416: F, t1318: F, t9436: F, t518: F, t5400: F, t577: F, t10015: F, t4484: F, t1328: F, t2098: F, t3965: F, t3966: F) -> (F, F, F, F, F, F) {
    let t13009 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4753 * t4895;
    let t13011 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3416 * t4895;
    let t13013 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1318 * t9436;
    let t13014 = t5400 * t518;
    let t13016 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t13014 * t577;
    let t13018 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t10015 * t4484;
    let t13022 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3965 * t3966 * t2098 * t1328;
    (t13009, t13011, t13013, t13016, t13018, t13022)
}
