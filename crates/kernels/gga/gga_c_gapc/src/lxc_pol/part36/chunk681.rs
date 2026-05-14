//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 681/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk681<F: Float>(t8709: F, t4052: F, t1030: F, t134: F, t5700: F, t442: F, t5963: F, t647: F, t1018: F, t568: F, t3080: F, t1044: F, t998: F, t169: F, t1019: F, t3017: F, t5983: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8710 = t8709 * M_PI;
    let t8711 = t4052 * t8710;
    let t8712 = t1030 * t8711;
    let t8714 = t134 * t5700;
    let t8715 = t8714 * t442;
    let t8716 = t5963 * t647 * t8715;
    let t8717 = t8712 * t8716;
    let t8719 = t1018 * t568;
    let t8720 = t3080 * t8719;
    let t8724 = t998 * t1044;
    let t8725 = t169 * t8724;
    let t8726 = t8725 * t1019;
    let t8728 = t3017 * t5983;
    (t8710, t8711, t8715, t8716, t8717, t8720, t8725, t8726, t8728)
}
