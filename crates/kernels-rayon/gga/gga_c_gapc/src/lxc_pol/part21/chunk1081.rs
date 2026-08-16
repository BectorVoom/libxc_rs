//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1081/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1081(t1084: f64, t15610: f64, t33411: f64, t1734: f64, t8709: f64, t15516: f64, t3708: f64, t9563: f64, t9934: f64, t33387: f64, t33390: f64, t33394: f64, t33396: f64, t33402: f64, t33405: f64, t33407: f64, t33409: f64) -> (f64, f64) {
    let t33413 = t1084 * t33411 * t15610;
    let t33415 = t1734 * t8709;
    let t33417 = t1084 * t33415 * t15516;
    let t33420 = t9563 * t3708 * t9934;
    let t33422 = -0.38647271295071362318e-6_f64 * t33387 + 0.33764099580923002116e-6_f64 * t33390 - 0.4976888445083044254e-7_f64 * t33394 - 0.52756405595192190805e-8_f64 * t33396 + 0.22098551499687900009e-8_f64 * t33402 - 0.21102562238076876322e-7_f64 * t33405 - 0.18115908419564701086e-6_f64 * t33407 + 0.52756405595192190805e-8_f64 * t33409 + 0.168651611569216142e-8_f64 * t33413 + 0.27665946779727057415e-8_f64 * t33417 - 0.33147827249531850014e-7_f64 * t33420;
    (t33415, t33422)
}
