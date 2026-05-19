//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 857/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk857<F: Float>(t2727: F, t343: F, t8145: F, t928: F, t328: F, t8148: F, t4606: F, t5021: F, t8141: F, t8143: F, t8146: F, t8149: F, t8155: F) -> (F, F, F, F) {
    let t8157 = t2727 * t343;
    let t8159 = t928 * t8145;
    let t8161 = t328 * t8148;
    let t8164 = -F::cast_from(2.8769444444444443_f64) * t8141 + F::cast_from(27.618666666666666_f64) * t8143 - F::cast_from(10.229135802469136_f64) * t8146 + F::cast_from(8.950493827160495_f64) * t8149 + F::cast_from(3.131074074074074_f64) * t4606 + F::new(0.0366775) * t8155 - F::new(0.58684) * t8157 + F::cast_from(0.6520444444444444_f64) * t8159 + F::cast_from(0.5705388888888889_f64) * t8161 + F::cast_from(1.3490888888888888_f64) * t5021;
    (t8157, t8159, t8161, t8164)
}
