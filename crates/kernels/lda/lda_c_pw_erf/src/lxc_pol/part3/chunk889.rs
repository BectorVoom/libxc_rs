//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 889/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk889<F: Float>(t432: F, t4606: F, t416: F, t118: F, t1184: F, t119: F, t120: F, t3262: F, t411: F, t3273: F, t156: F, t3291: F, t426: F) -> (F, F, F, F, F, F) {
    let t8871 = F::cast_from(2.5390814814814813_f64) * t432 * t4606;
    let t8873 = F::cast_from(5.052141975308642_f64) * t416 * t4606;
    let t8877 = F::new(70.0) / F::new(81.0) * t118 * t119 * t1184 * t120;
    let t8879 = t119 * t3262 * t411;
    let t8880 = t3273 * t8879;
    let t8884 = t426 * t156 * t3291;
    (t8871, t8873, t8877, t8879, t8880, t8884)
}
