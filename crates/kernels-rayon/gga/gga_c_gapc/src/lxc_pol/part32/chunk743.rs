//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 743/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk743(t8709: f64, t4052: f64, t1030: f64, t134: f64, t5700: f64, t442: f64, t5963: f64, t647: f64, t1018: f64, t568: f64, t3080: f64, t8668: f64, t8671: f64, t8678: f64, t8682: f64, t8688: f64, t8691: f64, t8694: f64, t8698: f64, t8702: f64, t8705: f64, t8707: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t8710 = t8709 * pi;
    let t8711 = t4052 * t8710;
    let t8712 = t1030 * t8711;
    let t8714 = t134 * t5700;
    let t8715 = t8714 * t442;
    let t8716 = t5963 * t647 * t8715;
    let t8717 = t8712 * t8716;
    let t8719 = t1018 * t568;
    let t8720 = t3080 * t8719;
    let t8722 = -0.39192950730437765221e-2_f64 * t8668 - 0.20241536458333333334e-4_f64 * t8671 - 0.29518907335069444446e-5_f64 * t8678 - 0.29518907335069444446e-5_f64 * t8682 + 0.21116891557347933848e-6_f64 * t8688 - 0.11594181388521408695e-4_f64 * t8691 - 0.13900948042322754167e-2_f64 * t8694 + 0.27801896084645508334e-2_f64 * t8698 + 0.6487109086417285278e-2_f64 * t8702 - 0.28985453471303521736e-5_f64 * t8705 + 0.28985453471303521736e-5_f64 * t8707 - 0.35904819748957283431e-8_f64 * t8717 + 0.67471788194444444446e-5_f64 * t8720;
    (t8710, t8711, t8715, t8716, t8722)
}
