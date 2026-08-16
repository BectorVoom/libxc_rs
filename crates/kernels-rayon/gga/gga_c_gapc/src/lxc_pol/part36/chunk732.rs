//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 732/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk732(t8709: f64, t4052: f64, t1030: f64, t134: f64, t5700: f64, t442: f64, t5963: f64, t647: f64, t1018: f64, t568: f64, t3080: f64, t1044: f64, t998: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
    let t8724 = t998 * t1044;
    (t8710, t8711, t8715, t8716, t8717, t8720, t8724)
}
