//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 972/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk972(t19020: f64, t2665: f64, t446: f64, t17744: f64, t835: f64, t17780: f64, t3281: f64, t4973: f64, t824: f64, t10279: f64, t10400: f64, t14636: f64, t14638: f64, t14640: f64, t14658: f64, t14684: f64, t14718: f64, t14903: f64, t15111: f64, t15116: f64, t18999: f64, t19004: f64, t19008: f64, t19013: f64, t19018: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19021 = t2665 * t19020;
    let t19022 = t446 * t19021;
    let t19024 = t835 * t17744;
    let t19025 = t446 * t19024;
    let t19027 = t835 * t17780;
    let t19028 = t3281 * t19027;
    let t19030 = t4973 * t824;
    let t19031 = t2665 * t19030;
    let t19032 = t446 * t19031;
    let t19034 = -t14636 - t14638 + t14640 - t14658 - t14684 - 2.0_f64 / 27.0_f64 * t10400 - 2.0_f64 / 81.0_f64 * t10279 - t15111 - 2.0_f64 / 27.0_f64 * t14718 - 2.0_f64 / 9.0_f64 * t18999 - 2.0_f64 / 9.0_f64 * t19004 + 2.0_f64 / 27.0_f64 * t19008 - t15116 + t14903 + t19013 / 18.0_f64 - t19018 / 9.0_f64 - t19022 / 9.0_f64 - t19025 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t19028 + t19032 / 18.0_f64;
    (t19022, t19025, t19028, t19030, t19032, t19034)
}
