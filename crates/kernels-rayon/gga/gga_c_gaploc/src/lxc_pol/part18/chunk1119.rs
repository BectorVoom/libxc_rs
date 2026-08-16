//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1119/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1119(t29439: f64, t9649: f64, t123: f64, t23092: f64, t2563: f64, t9647: f64, t1841: f64, t9752: f64, t1843: f64, t21456: f64, t7064: f64, t21461: f64) -> (f64, f64, f64, f64, f64) {
    let t29441 = 0.3845263115071112142e-2_f64 * t29439 * t9649;
    let t29445 = 0.3845263115071112142e-2_f64 * t9647 * t23092 * t123 * t2563;
    let t29447 = 0.17090058289204942853e-2_f64 * t1841 * t9752;
    let t29450 = 0.1281754371690370714e-2_f64 * t7064 * t1843 * t21456;
    let t29453 = 0.64087718584518535698e-3_f64 * t7064 * t1843 * t21461;
    (t29441, t29445, t29447, t29450, t29453)
}
