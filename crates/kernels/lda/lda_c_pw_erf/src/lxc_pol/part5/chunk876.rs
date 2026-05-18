//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 876/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk876<F: Float>(t1029: F, t400: F, t8170: F, t8171: F, t1059: F, t2987: F, t1063: F, t2694: F, t296: F, t905: F, t3136: F, t334: F) -> (F, F, F, F, F, F) {
    let t8173 = t1029 * t1029;
    let t8174 = F::new(1.0) / t8173;
    let t8177 = F::new(91080.98259910992) * t400 * t8170 * t8171 * t8174;
    let t8180 = t1059 * t2987;
    let t8184 = F::new(4.740006021527056) * t2694 * t1063 * t296;
    let t8185 = t905 * t905;
    let t8188 = F::new(24.0) * t3136 * t8185 * t334;
    (t8174, t8177, t8180, t8184, t8185, t8188)
}
