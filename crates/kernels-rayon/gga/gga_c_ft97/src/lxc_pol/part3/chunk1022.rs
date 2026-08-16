//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1022/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1022(t10688: f64, t5309: f64, t296: f64, t5330: f64, t684: f64, t10703: f64, t15229: f64, t19002: f64, t10553: f64, t10640: f64, t14715: f64, t14718: f64, t14921: f64, t14922: f64, t14923: f64, t14929: f64, t14936: f64, t18999: f64, t19732: f64) -> (f64, f64, f64, f64, f64) {
    let t19810 = t10688 * t5309;
    let t19811 = t296 * t19810;
    let t19815 = t5330 * t684;
    let t19816 = t10703 * t19815;
    let t19819 = t15229 * t19002;
    let t19826 = -t14921 - t14922 + t14923 - t14929 - t14936 - t10553 + t19732 / 2.0_f64 - t10640 - 8.0_f64 / 27.0_f64 * t14715 - 4.0_f64 / 9.0_f64 * t14718 - 4.0_f64 / 3.0_f64 * t18999;
    (t19810, t19811, t19816, t19819, t19826)
}
