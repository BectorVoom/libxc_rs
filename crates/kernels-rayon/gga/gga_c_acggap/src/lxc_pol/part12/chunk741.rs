//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 741/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk741(t7864: f64, t1090: f64, t1181: f64, t604: f64, t7575: f64, t1096: f64, t1165: f64, t7351: f64, t3034: f64, t614: f64, t2130: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7865 = 7.0_f64 / 144.0_f64 * t7864;
    let t7867 = t1181 * t604 * t1090;
    let t7868 = t7575 * t7867;
    let t7871 = t1165 * t7351 * t1096;
    let t7872 = t7575 * t7871;
    let t7884 = t614 * t3034;
    let t7885 = t7884 * t2130;
    (t7865, t7867, t7868, t7871, t7872, t7884, t7885)
}
