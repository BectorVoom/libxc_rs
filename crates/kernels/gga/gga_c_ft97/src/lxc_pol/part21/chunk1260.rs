//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1260/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1260<F: Float>(t23405: F, t30137: F, t106200: F, t1349: F, t1360: F, t1389: F, t16659: F, t16932: F, t17507: F, t1969: F, t23408: F, t23413: F, t26783: F, t27411: F, t28: F, t30106: F, t30169: F, t30560: F, t3313: F, t3414: F, t3450: F, t4458: F, t50260: F, t564: F, t5772: F, t614: F, t6723: F, t9432: F) -> (F,) {
    let t119533 = t23405 * t30137;
    let t119541 = t1349 * t28 * t30106 * t614 / 6.0 + 2.0 * t5772 * t9432 * t26783 * t3450 + t1349 * t28 * t1360 * t17507 / 6.0 - t16932 * t1389 - t16659 * t1389 - 2.0 * t3414 * t6723 - 2.0 * t3313 * t6723 - 8.0 / 27.0 * t106200 - t564 * t30560 - 24.0 * t50260 * t27411 + t119533 / 27.0 + t23413 * t30169 / 9.0 + t5772 * t1969 * t23408 * t4458 / 9.0;
    (t119541,)
}
