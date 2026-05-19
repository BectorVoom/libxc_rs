//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 673/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk673<F: Float>(t168: F, t270: F, t2782: F, t1143: F, t466: F, t148: F, t2929: F, t1159: F, t242: F, t632: F, t695: F, t1112: F, t145: F) -> (F, F, F, F, F, F) {
    let t4091 = F::cast_from(0.19455129084526285_f64) * t168 * t2782 * t270;
    let t4092 = t466 * t1143;
    let t4095 = F::cast_from(0.0837628205355044_f64) * t148 * t2929;
    let t4096 = t1159 * t242;
    let t4099 = F::cast_from(0.5025769232130264_f64) * t695 * t632;
    let t4100 = t145 * t1112;
    (t4091, t4092, t4095, t4096, t4099, t4100)
}
