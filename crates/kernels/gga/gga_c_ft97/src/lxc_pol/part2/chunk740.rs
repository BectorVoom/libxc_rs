//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 740/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk740<F: Float>(t12338: F, t13208: F, t167: F, t9114: F, t12590: F, t3455: F, t379: F, t9144: F, t2179: F, t582: F, t3483: F, t13187: F, t13190: F, t13192: F, t13196: F, t13198: F, t13201: F, t13205: F, t1901: F, t3281: F, t446: F, t9449: F, t9451: F, t9453: F, t9457: F) -> (F,) {
    let t13209 = t13208 * t12338;
    let t13212 = t9114 * t167;
    let t13213 = t13212 * t12590;
    let t13216 = t3455 * t379;
    let t13217 = t9144 * t13216;
    let t13220 = t582 * t2179;
    let t13221 = t3483 * t379;
    let t13222 = t13220 * t13221;
    let t13225 = -2.0 / 9.0 * t9449 - 2.0 / 9.0 * t9451 - 4.0 / 27.0 * t13187 + t13190 + 2.0 / 3.0 * t446 * t13192 + t13196 - 2.0 / 9.0 * t3281 * t13198 - 4.0 / 27.0 * t13201 + t9453 / 27.0 - 2.0 / 9.0 * t1901 * t13205 - 4.0 / 9.0 * t1901 * t13209 + 4.0 / 27.0 * t1901 * t13213 - 2.0 / 9.0 * t1901 * t13217 - 4.0 / 9.0 * t1901 * t13222 - t9457;
    (t13225,)
}
