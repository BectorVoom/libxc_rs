//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 732/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk732(t1775: f64, t4220: f64, t2347: f64, t852: f64, t2360: f64, t14635: f64, t14637: f64, t14639: f64, t14657: f64, t14683: f64, t14715: f64, t14895: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15028 = 4.0_f64 / 3.0_f64 * t1775 * t4220;
    let t15042 = t852 * t2347;
    let t15047 = t852 * t2360;
    let t15081 = 2.0_f64 / 27.0_f64 * t14635;
    let t15082 = 4.0_f64 / 27.0_f64 * t14637;
    let t15083 = 4.0_f64 / 81.0_f64 * t14639;
    let t15089 = 2.0_f64 / 27.0_f64 * t14657;
    let t15096 = 4.0_f64 / 9.0_f64 * t14683;
    let t15111 = 4.0_f64 / 81.0_f64 * t14715;
    let t15116 = 4.0_f64 / 27.0_f64 * t14895;
    (t15028, t15042, t15047, t15081, t15082, t15083, t15089, t15096, t15111, t15116)
}
