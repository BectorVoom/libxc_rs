//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1311/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1311<F: Float>(t34378: F, t34380: F, t34383: F, t34386: F, t34388: F, t34390: F, t34394: F, t34397: F, t34400: F, t34403: F, t34406: F, t34410: F, t34413: F, t34417: F, t34421: F, t34424: F, t34426: F, t34428: F, t34433: F, t34436: F, t34439: F, t34442: F) -> (F, F) {
    let t38131 = F::new(0.69504740211613770836e-3) * t34378 + F::new(0.90040494913303489552e-7) * t34380 + F::new(0.90040494913303489554e-6) * t34383 + F::new(0.90040494913303489552e-7) * t34386 + F::new(0.90040494913303489554e-6) * t34388 - F::new(0.4637672555408563478e-4) * t34390 - F::new(0.13900948042322754167e-2) * t34394 - F::new(0.8244751209615223961e-5) * t34397 - F::new(0.86880925264517213544e-4) * t34400 - F::new(0.86880925264517213544e-4) * t34403 - F::new(0.43440462632258606772e-4) * t34406;
    let t38143 = F::new(0.13259557375557346398e-6) * t34410 + F::new(0.19666550313313802087e-7) * t34413 - F::new(0.17809610181709224597e-4) * t34417 - F::new(0.97834092881944444454e-4) * t34421 + F::new(0.50004799207799907351e-2) * t34424 + F::new(0.6487109086417285278e-2) * t34426 + F::new(0.6487109086417285278e-2) * t34428 - F::new(0.8839704917038230932e-7) * t34433 - F::new(0.30013498304434496518e-7) * t34436 + F::new(0.43440462632258606772e-4) * t34439 - F::new(0.4637672555408563478e-4) * t34442;
    (t38131, t38143)
}
