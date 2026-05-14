//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 689/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk689<F: Float>(t8709: F, t4052: F, t1030: F, t134: F, t5700: F, t442: F, t5963: F, t647: F, t1018: F, t568: F, t3080: F, t8668: F, t8671: F, t8678: F, t8682: F, t8688: F, t8691: F, t8694: F, t8698: F, t8702: F, t8705: F, t8707: F) -> (F, F, F, F, F) {
    let t8710 = t8709 * M_PI;
    let t8711 = t4052 * t8710;
    let t8712 = t1030 * t8711;
    let t8714 = t134 * t5700;
    let t8715 = t8714 * t442;
    let t8716 = t5963 * t647 * t8715;
    let t8717 = t8712 * t8716;
    let t8719 = t1018 * t568;
    let t8720 = t3080 * t8719;
    let t8722 = -0.39192950730437765221e-2 * t8668 - 0.20241536458333333334e-4 * t8671 - 0.29518907335069444446e-5 * t8678 - 0.29518907335069444446e-5 * t8682 + 0.21116891557347933848e-6 * t8688 - 0.11594181388521408695e-4 * t8691 - 0.13900948042322754167e-2 * t8694 + 0.27801896084645508334e-2 * t8698 + 0.6487109086417285278e-2 * t8702 - 0.28985453471303521736e-5 * t8705 + 0.28985453471303521736e-5 * t8707 - 0.35904819748957283431e-8 * t8717 + 0.67471788194444444446e-5 * t8720;
    (t8710, t8711, t8715, t8716, t8722)
}
