//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1080/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1080<F: Float>(t1084: F, t15610: F, t33411: F, t1734: F, t8709: F, t15516: F, t3708: F, t9563: F, t9934: F, t33387: F, t33390: F, t33394: F, t33396: F, t33402: F, t33405: F, t33407: F, t33409: F) -> (F, F) {
    let t33413 = t1084 * t33411 * t15610;
    let t33415 = t1734 * t8709;
    let t33417 = t1084 * t33415 * t15516;
    let t33420 = t9563 * t3708 * t9934;
    let t33422 = -F::cast_from(0.38647271295071362318e-6_f64) * t33387 + F::cast_from(0.33764099580923002116e-6_f64) * t33390 - F::cast_from(0.4976888445083044254e-7_f64) * t33394 - F::cast_from(0.52756405595192190805e-8_f64) * t33396 + F::cast_from(0.22098551499687900009e-8_f64) * t33402 - F::cast_from(0.21102562238076876322e-7_f64) * t33405 - F::cast_from(0.18115908419564701086e-6_f64) * t33407 + F::cast_from(0.52756405595192190805e-8_f64) * t33409 + F::cast_from(0.168651611569216142e-8_f64) * t33413 + F::cast_from(0.27665946779727057415e-8_f64) * t33417 - F::cast_from(0.33147827249531850014e-7_f64) * t33420;
    (t33415, t33422)
}
