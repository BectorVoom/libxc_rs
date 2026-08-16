//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1397/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1397<F: Float>(t34346: F, t34351: F, t34353: F, t34356: F, t34359: F, t34364: F, t34367: F, t34370: F, t34361: F, t34373: F, t36906: F, t34378: F, t34380: F, t34383: F, t34386: F, t34388: F, t34390: F, t34394: F, t34397: F, t34400: F, t34403: F, t34406: F) -> (F, F) {
    let t36907 = F::cast_from(0.43440462632258606772e-4_f64) * t34346;
    let t36908 = F::cast_from(0.50680539737635041234e-3_f64) * t34351;
    let t36909 = F::cast_from(0.40483072916666666669e-4_f64) * t34353;
    let t36910 = F::cast_from(0.57920616843011475696e-5_f64) * t34356;
    let t36911 = F::cast_from(0.57920616843011475696e-5_f64) * t34359;
    let t36913 = F::cast_from(0.1011909669415296852e-6_f64) * t34364;
    let t36914 = F::cast_from(0.2318836277704281739e-4_f64) * t34367;
    let t36915 = F::cast_from(0.4637672555408563478e-4_f64) * t34370;
    let t36917 = t36906 + t36907 - t36908 - t36909 + t36910 + t36911 - F::cast_from(0.5691280480400994668e-7_f64) * t34361 + t36913 + t36914 - t36915 + F::cast_from(0.68360384691762319206e-5_f64) * t34373;
    let t36930 = F::cast_from(0.34752370105806885418e-3_f64) * t34378 + F::cast_from(0.45020247456651744776e-7_f64) * t34380 + F::cast_from(0.45020247456651744776e-6_f64) * t34383 + F::cast_from(0.45020247456651744776e-7_f64) * t34386 + F::cast_from(0.45020247456651744776e-6_f64) * t34388 - F::cast_from(0.2318836277704281739e-4_f64) * t34390 - F::cast_from(0.69504740211613770836e-3_f64) * t34394 - F::cast_from(0.41223756048076119805e-5_f64) * t34397 - F::cast_from(0.43440462632258606772e-4_f64) * t34400 - F::cast_from(0.43440462632258606772e-4_f64) * t34403 - F::cast_from(0.21720231316129303386e-4_f64) * t34406;
    (t36917, t36930)
}
