//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 867/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk867(t8045: f64, t9260: f64, t12862: f64, t4349: f64, t605: f64, t10298: f64, t6556: f64, t12856: f64, t17288: f64, t2801: f64, t31428: f64, t1016: f64, t1382: f64, t9588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42475 = 2.0_f64 * t8045 * t9260;
    let t42481 = 6.0_f64 * t4349 * t12862 * t605;
    let t42483 = 4.0_f64 * t6556 * t10298;
    let t42485 = 6.0_f64 * t17288 * t12856;
    let t42487 = 2.0_f64 * t31428 * t2801;
    let t42491 = 2.0_f64 * t1382 * t1016 * t9588;
    (t42475, t42481, t42483, t42485, t42487, t42491)
}
