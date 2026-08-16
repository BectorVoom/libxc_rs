//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 927/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk927<F: Float>(t10030: F, t3979: F, t3994: F, t515: F, t174: F, t205: F, t9810: F, t325: F, t3648: F, t1332: F, t1350: F, t3640: F) -> (F, F, F, F, F, F, F) {
    let t10031 = t10030 * t3979;
    let t10039 = t3994 * t515;
    let t10042 = t174 * t9810 * t205;
    let t10043 = F::cast_from(0.01959135802469136_f64) * t10042;
    let t10053 = t325 * t3648;
    let t10056 = F::cast_from(1.0_f64) / t1350 / t1332;
    let t10066 = t325 * t3640;
    (t10031, t10039, t10042, t10043, t10053, t10056, t10066)
}
