//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 736/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk736(t1882: f64, t4276: f64, t4280: f64, t2681: f64, t309: f64, t1212: f64, t870: f64, t4147: f64, t8392: f64, t4257: f64, t4262: f64, t10580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15334 = 2.0_f64 / 9.0_f64 * t1882 * t4276;
    let t15336 = 2.0_f64 / 9.0_f64 * t1882 * t4280;
    let t15369 = t2681 * t309;
    let t15370 = t870 * t1212;
    let t15376 = 2.0_f64 / 27.0_f64 * t8392 * t4147;
    let t15382 = 2.0_f64 / 27.0_f64 * t8392 * t4257;
    let t15384 = 2.0_f64 / 27.0_f64 * t8392 * t4262;
    let t15385 = t10580 * t309;
    (t15334, t15336, t15369, t15370, t15376, t15382, t15384, t15385)
}
