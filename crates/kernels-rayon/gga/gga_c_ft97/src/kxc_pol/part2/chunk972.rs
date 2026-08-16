//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 972/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk972(t15073: f64, t871: f64, t14635: f64, t14637: f64, t14639: f64, t14619: f64, t14622: f64, t14626: f64, t14630: f64, t14633: f64, t14642: f64, t14645: f64, t14650: f64) -> (f64, f64) {
    let t15074 = t871 * t15073;
    let t15081 = 2.0_f64 / 27.0_f64 * t14635;
    let t15082 = 4.0_f64 / 27.0_f64 * t14637;
    let t15083 = 4.0_f64 / 81.0_f64 * t14639;
    let t15087 = -8.0_f64 / 9.0_f64 * t14619 + 8.0_f64 / 27.0_f64 * t14622 + t14626 / 9.0_f64 - 4.0_f64 / 9.0_f64 * t14630 + 2.0_f64 / 9.0_f64 * t14633 - t15081 - t15082 + t15083 - 2.0_f64 / 27.0_f64 * t14642 - 10.0_f64 / 81.0_f64 * t14645 - 2.0_f64 / 9.0_f64 * t14650;
    (t15074, t15087)
}
