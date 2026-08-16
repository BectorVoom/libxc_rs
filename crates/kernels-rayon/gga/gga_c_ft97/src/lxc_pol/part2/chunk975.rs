//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 975/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk975(t14902: f64, t10243: f64, t10658: f64, t14718: f64, t14892: f64, t14899: f64, t15058: f64, t15062: f64, t15065: f64, t15069: f64, t15116: f64, t15087: f64, t15099: f64, t15112: f64) -> f64 {
    let t15118 = 2.0_f64 / 9.0_f64 * t14902;
    let t15123 = -22.0_f64 / 27.0_f64 * t14718 - 2.0_f64 / 27.0_f64 * t10243 - t14892 / 3.0_f64 - t15116 + 2.0_f64 / 9.0_f64 * t14899 + t15118 + t15058 / 6.0_f64 - t10658 - t15062 / 6.0_f64 - t15065 / 12.0_f64 + t15069 / 8.0_f64;
    let t15125 = t15087 + t15099 + t15112 + t15123;
    t15125
}
