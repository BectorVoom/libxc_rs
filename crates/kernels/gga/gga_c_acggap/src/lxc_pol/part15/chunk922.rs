//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 922/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk922<F: Float>(t30248: F, t532: F, t537: F, t7637: F, t8859: F, t1576: F, t7614: F, t1181: F, t5249: F, t599: F, t7493: F, t4718: F, t604: F, t7426: F, t31349: F, t3360: F, t4284: F) -> (F, F, F, F, F, F, F) {
    let t36231 = t30248 * t532;
    let t36236 = t30248 * t537;
    let t36238 = t7637 * t8859;
    let t36240 = t7614 * t1576;
    let t36273 = t7493 * t1181 * t599 * t5249;
    let t36283 = t7426 * t1181 * t604 * t4718;
    let t36286 = t3360 * t31349 * t4284;
    (t36231, t36236, t36238, t36240, t36273, t36283, t36286)
}
