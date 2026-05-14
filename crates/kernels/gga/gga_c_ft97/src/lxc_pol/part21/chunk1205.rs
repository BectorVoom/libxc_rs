//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1205/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1205<F: Float>(t1882: F, t29849: F, t103305: F, t103343: F, t103849: F, t110: F, t11490: F, t116136: F, t116543: F, t11810: F, t15994: F, t16198: F, t16203: F, t1871: F, t1901: F, t22943: F, t26001: F, t26061: F, t26166: F, t29706: F, t29789: F, t29910: F, t3214: F, t432: F, t446: F, t452: F, t488: F, t499: F, t83: F, t8372: F, t986: F) -> (F,) {
    let t117680 = t1882 * t29849;
    let t117699 = 2.0 / 9.0 * t1901 * t8372 * t29910 + 2.0 / 3.0 * t446 * t452 * t26061 * t3214 + 4.0 / 3.0 * t446 * t1871 * t986 * t26001 + 4.0 / 3.0 * t446 * t1871 * t499 * t29706 + 4.0 / 3.0 * t446 * t1871 * t110 * t116543 + t103305 - t446 * t83 * t116136 / 3.0 - 2.0 / 3.0 * t446 * t452 * t22943 * t15994 + 2.0 / 81.0 * t117680 - 4.0 / 3.0 * t1901 * t11490 * t26166 * t16203 + 4.0 / 3.0 * t1901 * t11810 * t26166 * t15994 + 2.0 * t1901 * t11490 * t103849 * t16198 - t103343 + t446 * t452 * t488 * t29789 * t432 / 3.0;
    (t117699,)
}
