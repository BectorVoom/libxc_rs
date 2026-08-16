//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 345/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk345<F: Float>(t1217: F, t265: F, t665: F, t668: F, t108: F, t659: F, t661: F, t92: F, t93: F, t940: F, t945: F, t951: F, t954: F) -> (F, F, F) {
    let t1219 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t265 * t1217;
    let t1220 = t665 * t668;
    let t1231 = (F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t92 * t940 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t659 * t945 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t93 * t951 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t661 * t954) * t108;
    (t1219, t1220, t1231)
}
