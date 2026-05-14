//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1182/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1182<F: Float>(t30009: F, t8392: F, t102533: F, t102543: F, t102549: F, t11468: F, t11552: F, t11593: F, t116281: F, t116320: F, t116425: F, t116451: F, t116817: F, t11854: F, t11863: F, t16006: F, t16155: F, t16160: F, t16177: F, t16182: F, t1901: F, t23265: F, t26435: F, t29727: F, t3113: F, t379: F, t446: F, t447: F, t5691: F, t8557: F) -> (F,) {
    let t116922 = t8392 * t30009;
    let t116937 = -4.0 / 9.0 * t1901 * t11863 * t116320 - t446 * t447 * t29727 * t379 / 9.0 + t102533 - t102543 - t102549 - 4.0 / 9.0 * t1901 * t11854 * t116817 * t3113 - 4.0 / 9.0 * t1901 * t11854 * t23265 * t16177 - 8.0 / 9.0 * t11593 * t11854 * t23265 * t16182 + 2.0 / 9.0 * t1901 * t8557 * t26435 * t16155 + 4.0 / 9.0 * t1901 * t11854 * t26435 * t16160 + 2.0 / 27.0 * t116922 - 2.0 / 9.0 * t1901 * t11863 * t116281 - t1901 * t8557 * t5691 * t16006 / 9.0 - 2.0 / 9.0 * t1901 * t11468 * t116451 + 2.0 / 27.0 * t1901 * t11552 * t116425;
    (t116937,)
}
