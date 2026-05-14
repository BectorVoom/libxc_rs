//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1245/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1245<F: Float>(t26214: F, t8392: F, t487: F, t6454: F, t6531: F, t8232: F, t11593: F, t11595: F, t11623: F, t11810: F, t11902: F, t1647: F, t1871: F, t1876: F, t1901: F, t1902: F, t22970: F, t23129: F, t23172: F, t23201: F, t23206: F, t23265: F, t23327: F, t25846: F, t26356: F, t3238: F, t3281: F, t446: F, t447: F, t452: F, t488: F, t499: F, t5710: F, t91754: F, t91760: F, t942: F, t986: F) -> (F,) {
    let t103107 = 2.0 / 27.0 * t8392 * t26214;
    let t103108 = t487 * t6454;
    let t103121 = t8232 * t6531;
    let t103135 = 2.0 / 9.0 * t3281 * t447 * t488 * t23265 - 2.0 / 9.0 * t1901 * t1902 * t26356 * t1647 + 4.0 / 9.0 * t11593 * t23327 * t11595 - 2.0 / 3.0 * t446 * t1871 * t5710 * t11623 + t91754 / 27.0 - 2.0 / 9.0 * t91760 - 2.0 / 9.0 * t1901 * t11902 * t23172 - t103107 - 4.0 / 3.0 * t1901 * t11810 * t103108 * t1876 - t446 * t452 * t23129 * t942 / 3.0 - 2.0 / 3.0 * t446 * t452 * t499 * t25846 - 4.0 / 27.0 * t103121 + t446 * t452 * t3238 * t23206 / 3.0 + 4.0 / 3.0 * t446 * t1871 * t986 * t22970 + 2.0 / 3.0 * t446 * t452 * t3238 * t23201;
    (t103135,)
}
