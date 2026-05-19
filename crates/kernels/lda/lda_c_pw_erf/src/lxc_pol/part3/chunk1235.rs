//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1235/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1235<F: Float>(t8985: F, t9003: F, t9005: F, t9009: F, t9011: F, t9015: F, t9017: F, t9094: F, t9096: F, t9098: F, t9100: F, t9104: F, t9110: F) -> (F, F, F, F, F, F, F, F) {
    let t14566 = F::cast_from(0.9743416666666667_f64) * t8985;
    let t14567 = F::cast_from(4.5469277777777775_f64) * t9003;
    let t14568 = F::cast_from(1.9486833333333333_f64) * t9005;
    let t14569 = F::new(2.923025) * t9009;
    let t14570 = F::cast_from(3.8973666666666666_f64) * t9011;
    let t14571 = F::cast_from(1.9486833333333333_f64) * t9015;
    let t14572 = F::cast_from(3.8973666666666666_f64) * t9017;
    let t14579 = t14566 + t14567 - t14568 + t14569 + t14570 - t14571 - t14572 - F::new(5.172765) * t9094 + F::cast_from(5.364348888888889_f64) * t9096 - F::cast_from(2.2990066666666666_f64) * t9098 + F::cast_from(0.5747516666666667_f64) * t9100 + F::new(6.89702) * t9104 + F::new(6.89702) * t9110;
    (t14566, t14567, t14568, t14569, t14570, t14571, t14572, t14579)
}
