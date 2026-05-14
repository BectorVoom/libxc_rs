//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 647/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk647<F: Float>(t110: F, t20045: F, t447: F, t20023: F, t8577: F, t4551: F, t942: F, t1852: F, t452: F, t4589: F, t979: F, t83: F, t11550: F, t11578: F, t16192: F, t16213: F, t1901: F, t20226: F, t20230: F, t20233: F, t20236: F, t20240: F, t20244: F, t20248: F, t446: F) -> (F, F, F, F, F, F, F, F) {
    let t20256 = t447 * t110 * t20045;
    let t20260 = t8577 * t110 * t20023;
    let t20263 = t4551 * t942;
    let t20265 = t452 * t1852 * t20263;
    let t20268 = t979 * t4589;
    let t20269 = t1852 * t20268;
    let t20270 = t83 * t20269;
    let t20273 = 2.0 / 3.0 * t1901 * t20226 - 2.0 / 9.0 * t1901 * t20230 + 2.0 / 3.0 * t1901 * t20233 + 2.0 / 3.0 * t1901 * t20236 - 2.0 / 3.0 * t1901 * t20240 + 4.0 / 9.0 * t446 * t20244 + 2.0 / 3.0 * t446 * t20248 - 4.0 / 9.0 * t11550 - 2.0 / 9.0 * t16192 + 4.0 / 9.0 * t11578 + t16213 / 3.0 - t446 * t20256 / 9.0 - 10.0 / 81.0 * t446 * t20260 - 2.0 * t446 * t20265 + 2.0 * t446 * t20270;
    (t20256, t20260, t20263, t20265, t20268, t20269, t20270, t20273)
}
