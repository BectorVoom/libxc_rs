//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1254/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1254<F: Float>(t49562: F, t6708: F, t104175: F, t104477: F, t104478: F, t104512: F, t104519: F, t1058: F, t107703: F, t1349: F, t1557: F, t16666: F, t16671: F, t26561: F, t26805: F, t26811: F, t26817: F, t27426: F, t28: F, t30133: F, t3051: F, t3188: F, t3588: F, t4837: F, t5772: F, t5843: F, t609: F, t6579: F, t6580: F, t6587: F, t6616: F, t9439: F) -> (F, F) {
    let t119348 = t49562 * t6708;
    let t119384 = 8.0 * t119348 - 2.0 / 3.0 * t1349 * t28 * t104175 * t6587 - 12.0 * t9439 * t30133 * t609 - 2.0 / 27.0 * t5772 * t27426 * t1058 * t1557 * t3188 + 4.0 / 27.0 * t104512 - 2.0 / 3.0 * t104477 * t104478 * t16666 - 4.0 / 9.0 * t104477 * t107703 * t16671 + t1349 * t28 * t6616 * t3588 / 3.0 - t104519 - t26817 * t26805 / 9.0 - 2.0 / 9.0 * t6579 * t3051 * t26811 + t1349 * t28 * t5843 * t4837 / 6.0 + t6580 * t26561 / 3.0;
    (t119348, t119384)
}
