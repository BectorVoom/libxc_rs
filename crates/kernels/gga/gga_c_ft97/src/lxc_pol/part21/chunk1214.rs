//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1214/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1214<F: Float>(t29932: F, t8392: F, t103439: F, t103698: F, t103872: F, t11472: F, t11556: F, t116108: F, t1557: F, t1570: F, t16261: F, t16313: F, t16320: F, t16324: F, t16328: F, t1901: F, t25846: F, t26113: F, t26171: F, t3188: F, t446: F, t452: F, t47659: F, t47666: F, t488: F, t5630: F, t6557: F, t83: F, t91539: F, t93609: F, t93612: F, t93621: F, t942: F, t986: F) -> (F,) {
    let t118079 = t8392 * t29932;
    let t118085 = t103698 - 2.0 * t1901 * t26171 * t5630 * t16261 + 2.0 / 3.0 * t446 * t83 * t116108 + 4.0 / 9.0 * t47659 * t91539 * t16320 + 4.0 / 3.0 * t47659 * t103872 * t16324 + 8.0 / 9.0 * t47659 * t103439 * t16328 - 8.0 / 27.0 * t47666 * t103439 * t16313 + 2.0 / 3.0 * t446 * t452 * t488 * t26113 * t942 - 4.0 / 9.0 * t1901 * t11472 * t6557 * t1570 * t3188 + 4.0 / 27.0 * t1901 * t11556 * t6557 * t1557 * t3188 - 4.0 / 27.0 * t93609 - 4.0 / 27.0 * t93612 - 4.0 / 27.0 * t93621 + 2.0 / 27.0 * t118079 - 2.0 / 3.0 * t446 * t452 * t986 * t25846;
    (t118085,)
}
