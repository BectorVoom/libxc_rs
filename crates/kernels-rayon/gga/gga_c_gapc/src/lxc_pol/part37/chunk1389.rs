//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1389/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1389(t34353: f64, t34356: f64, t34359: f64, t34364: f64, t34367: f64, t34370: f64, t34378: f64, t34380: f64, t34383: f64, t34386: f64, t34388: f64, t34390: f64, t34394: f64, t34397: f64, t34400: f64, t34403: f64, t34406: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36909 = 0.40483072916666666669e-4_f64 * t34353;
    let t36910 = 0.57920616843011475696e-5_f64 * t34356;
    let t36911 = 0.57920616843011475696e-5_f64 * t34359;
    let t36913 = 0.1011909669415296852e-6_f64 * t34364;
    let t36914 = 0.2318836277704281739e-4_f64 * t34367;
    let t36915 = 0.4637672555408563478e-4_f64 * t34370;
    let t36930 = 0.34752370105806885418e-3_f64 * t34378 + 0.45020247456651744776e-7_f64 * t34380 + 0.45020247456651744776e-6_f64 * t34383 + 0.45020247456651744776e-7_f64 * t34386 + 0.45020247456651744776e-6_f64 * t34388 - 0.2318836277704281739e-4_f64 * t34390 - 0.69504740211613770836e-3_f64 * t34394 - 0.41223756048076119805e-5_f64 * t34397 - 0.43440462632258606772e-4_f64 * t34400 - 0.43440462632258606772e-4_f64 * t34403 - 0.21720231316129303386e-4_f64 * t34406;
    (t36909, t36910, t36911, t36913, t36914, t36915, t36930)
}
