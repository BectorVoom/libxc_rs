//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 558/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk558(t4282: f64, t4284: f64, t1470: f64, t3409: f64, t1410: f64, t174: f64, t435: f64, t1549: f64, t1554: f64, t1558: f64, t1016: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4285 = t4282 * t4284;
    let t4288 = 0.40015750243531754508e-2_f64 * t3409 * t1470;
    let t4289 = t174 * t1410;
    let t4298 = t435 * t1410;
    let t4308 = 0.40015750243531754508e-2_f64 * t3409 * t1549;
    let t4310 = 0.40015750243531754508e-2_f64 * t3409 * t1554;
    let t4312 = 0.20007875121765877254e-2_f64 * t3409 * t1558;
    let t4313 = t1016 * t524;
    (t4285, t4288, t4289, t4298, t4308, t4310, t4312, t4313)
}
