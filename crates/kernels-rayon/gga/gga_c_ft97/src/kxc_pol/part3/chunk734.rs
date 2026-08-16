//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 734/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk734(t4241: f64, t681: f64, t89: f64, t1240: f64, t2770: f64, t848: f64, t1882: f64, t4305: f64, t319: f64, t871: f64, t4248: f64, t4301: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15190 = 2.0_f64 / 9.0_f64 * t89 * t681 * t4241;
    let t15191 = t2770 * t1240;
    let t15195 = t848 * t1240;
    let t15206 = 2.0_f64 / 9.0_f64 * t1882 * t4305;
    let t15229 = t2770 * t319;
    let t15254 = t848 * t871;
    let t15271 = 2.0_f64 / 9.0_f64 * t1882 * t4248;
    let t15273 = 2.0_f64 / 9.0_f64 * t1882 * t4301;
    (t15190, t15191, t15195, t15206, t15229, t15254, t15271, t15273)
}
