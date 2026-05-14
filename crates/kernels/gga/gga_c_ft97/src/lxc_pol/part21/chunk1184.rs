//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1184/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1184<F: Float>(t102664: F, t102694: F, t102697: F, t102698: F, t102706: F, t102708: F, t102723: F, t102783: F, t110: F, t11490: F, t116557: F, t116561: F, t11810: F, t11854: F, t16261: F, t16266: F, t1780: F, t1871: F, t1901: F, t22940: F, t23249: F, t29701: F, t3195: F, t446: F, t452: F, t4572: F, t4611: F, t499: F, t5717: F, t6524: F) -> (F,) {
    let t117017 = -4.0 / 9.0 * t1901 * t11854 * t102783 * t4611 - t102664 - 8.0 / 27.0 * t102694 - 4.0 / 27.0 * t1901 * t1780 * t6524 * t3195 + 2.0 / 3.0 * t446 * t452 * t22940 * t4572 - t102697 + 8.0 / 81.0 * t102698 + t102706 + t102708 - 2.0 / 3.0 * t1901 * t11810 * t5717 * t16261 - 2.0 / 3.0 * t1901 * t11490 * t23249 * t16266 + 2.0 / 3.0 * t446 * t1871 * t110 * t116561 + 2.0 / 3.0 * t446 * t1871 * t499 * t29701 + 2.0 / 3.0 * t446 * t1871 * t110 * t116557 + t102723;
    (t117017,)
}
