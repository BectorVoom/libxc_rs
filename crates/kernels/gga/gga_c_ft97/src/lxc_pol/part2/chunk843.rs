//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 843/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk843<F: Float>(t3483: F, t379: F, t13220: F, t13187: F, t13190: F, t13192: F, t13196: F, t13198: F, t13201: F, t13205: F, t13209: F, t13213: F, t13217: F, t1901: F, t3281: F, t446: F, t9449: F, t9451: F, t9453: F, t9457: F) -> F {
    let t13221 = t3483 * t379;
    let t13222 = t13220 * t13221;
    let t13225 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9449 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9451 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13187 + t13190 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t13192 + t13196 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t13198 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13201 + t9453 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13205 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t13209 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t13213 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13217 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t13222 - t9457;
    t13225
}
