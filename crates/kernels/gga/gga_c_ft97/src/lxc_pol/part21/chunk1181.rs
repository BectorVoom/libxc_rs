//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1181/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1181<F: Float>(t29940: F, t8392: F, t102508: F, t11468: F, t11593: F, t116285: F, t116289: F, t116292: F, t116338: F, t116429: F, t116438: F, t11854: F, t11863: F, t16160: F, t1901: F, t22986: F, t23031: F, t25933: F, t26134: F, t26305: F, t29602: F, t29641: F, t379: F, t39167: F, t46881: F, t47231: F, t47443: F, t60243: F, t60711: F, t8557: F) -> (F,) {
    let t116875 = t8392 * t29940;
    let t116893 = -2.0 / 9.0 * t1901 * t11854 * t29602 * t379 - 4.0 / 27.0 * t1901 * t46881 * t116429 - 2.0 / 27.0 * t1901 * t39167 * t22986 * t16160 - 4.0 / 9.0 * t1901 * t60711 * t26134 - 4.0 / 9.0 * t1901 * t11863 * t116438 + t102508 - 4.0 / 9.0 * t1901 * t60243 * t25933 - 2.0 / 9.0 * t1901 * t47443 * t26305 - 2.0 / 9.0 * t1901 * t11863 * t116338 + 4.0 / 27.0 * t116875 + 4.0 / 9.0 * t1901 * t11863 * t116285 + 2.0 / 9.0 * t1901 * t8557 * t23031 * t16160 + 2.0 / 3.0 * t1901 * t11468 * t116289 - 8.0 / 9.0 * t11593 * t11468 * t116292 - 4.0 / 9.0 * t1901 * t47231 * t29641;
    (t116893,)
}
