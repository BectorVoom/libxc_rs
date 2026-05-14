//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1231/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1231<F: Float>(t1332: F, t7763: F, t100190: F, t100319: F, t100323: F, t101621: F, t101626: F, t102465: F, t102467: F, t102469: F, t102471: F, t11437: F, t11468: F, t11552: F, t11556: F, t11593: F, t11594: F, t11599: F, t11854: F, t11863: F, t11982: F, t1557: F, t1901: F, t23265: F, t26440: F, t3188: F, t46881: F, t47799: F, t5743: F, t8557: F) -> (F,) {
    let t102487 = t1332 * t7763;
    let t102500 = -4.0 / 9.0 * t1901 * t11863 * t100323 - 2.0 / 9.0 * t1901 * t11863 * t100190 - 4.0 / 27.0 * t1901 * t46881 * t100319 + t102465 + t102467 + t102469 - t102471 + 4.0 / 27.0 * t1901 * t11556 * t5743 * t1557 * t3188 - 4.0 / 9.0 * t1901 * t11468 * t101621 + 4.0 / 27.0 * t1901 * t11552 * t101626 + 2.0 / 27.0 * t1901 * t11556 * t26440 * t11982 + 10.0 / 81.0 * t1901 * t47799 * t102487 * t11437 - 4.0 / 9.0 * t11593 * t8557 * t23265 * t11594 - 8.0 / 9.0 * t11593 * t11854 * t23265 * t11599;
    (t102500,)
}
