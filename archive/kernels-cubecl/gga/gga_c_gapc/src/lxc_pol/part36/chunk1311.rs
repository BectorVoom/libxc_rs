//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1311/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1311<F: Float>(t34378: F, t34380: F, t34383: F, t34386: F, t34388: F, t34390: F, t34394: F, t34397: F, t34400: F, t34403: F, t34406: F, t34410: F, t34413: F, t34417: F, t34421: F, t34424: F, t34426: F, t34428: F, t34433: F, t34436: F, t34439: F, t34442: F) -> (F, F) {
    let t38131 = F::cast_from(0.69504740211613770836e-3_f64) * t34378 + F::cast_from(0.90040494913303489552e-7_f64) * t34380 + F::cast_from(0.90040494913303489554e-6_f64) * t34383 + F::cast_from(0.90040494913303489552e-7_f64) * t34386 + F::cast_from(0.90040494913303489554e-6_f64) * t34388 - F::cast_from(0.4637672555408563478e-4_f64) * t34390 - F::cast_from(0.13900948042322754167e-2_f64) * t34394 - F::cast_from(0.8244751209615223961e-5_f64) * t34397 - F::cast_from(0.86880925264517213544e-4_f64) * t34400 - F::cast_from(0.86880925264517213544e-4_f64) * t34403 - F::cast_from(0.43440462632258606772e-4_f64) * t34406;
    let t38143 = F::cast_from(0.13259557375557346398e-6_f64) * t34410 + F::cast_from(0.19666550313313802087e-7_f64) * t34413 - F::cast_from(0.17809610181709224597e-4_f64) * t34417 - F::cast_from(0.97834092881944444454e-4_f64) * t34421 + F::cast_from(0.50004799207799907351e-2_f64) * t34424 + F::cast_from(0.6487109086417285278e-2_f64) * t34426 + F::cast_from(0.6487109086417285278e-2_f64) * t34428 - F::cast_from(0.8839704917038230932e-7_f64) * t34433 - F::cast_from(0.30013498304434496518e-7_f64) * t34436 + F::cast_from(0.43440462632258606772e-4_f64) * t34439 - F::cast_from(0.4637672555408563478e-4_f64) * t34442;
    (t38131, t38143)
}
