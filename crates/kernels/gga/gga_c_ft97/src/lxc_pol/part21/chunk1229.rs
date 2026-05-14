//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1229/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1229<F: Float>(t26564: F, t6580: F, t15772: F, t16011: F, t16950: F, t1969: F, t23408: F, t23413: F, t24102: F, t26817: F, t27417: F, t27420: F, t27421: F, t27423: F, t27426: F, t27427: F, t27429: F, t30149: F, t3565: F, t4454: F, t4462: F, t5772: F, t5773: F, t6708: F, t9049: F, t94175: F, t9439: F, t94976: F) -> (F,) {
    let t118598 = t6580 * t26564;
    let t118630 = 2.0 / 9.0 * t26817 * t27417 + 2.0 / 9.0 * t26817 * t27423 - 2.0 / 27.0 * t26817 * t27429 - t118598 / 9.0 + 2.0 / 27.0 * t94175 - t5772 * t1969 * t24102 * t4462 / 18.0 - t5772 * t1969 * t5773 * t15772 / 18.0 - t23413 * t30149 / 27.0 - t5772 * t9049 * t23408 * t4454 / 27.0 - 24.0 * t9439 * t6708 * t3565 + t5772 * t27420 * t27421 * t16011 / 9.0 - t5772 * t27426 * t27427 * t16011 / 27.0 + 2.0 / 27.0 * t5772 * t94976 * t27427 * t16950;
    (t118630,)
}
