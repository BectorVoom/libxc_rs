//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 859/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk859<F: Float>(t1029: F, t400: F, t8170: F, t8171: F, t1055: F, t2742: F, t1059: F, t2987: F, t1063: F, t2694: F, t296: F, t905: F) -> (F, F, F, F, F, F) {
    let t8173 = t1029 * t1029;
    let t8174 = F::new(1.0) / t8173;
    let t8177 = F::cast_from(91080.98259910992_f64) * t400 * t8170 * t8171 * t8174;
    let t8178 = t2742 * t1055;
    let t8180 = t1059 * t2987;
    let t8184 = F::cast_from(4.740006021527056_f64) * t2694 * t1063 * t296;
    let t8185 = t905 * t905;
    (t8174, t8177, t8178, t8180, t8184, t8185)
}
