//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 849/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk849<F: Float>(t3677: F, t7742: F, t2305: F, t920: F, t1080: F, t13256: F, t13260: F, t13263: F, t13268: F, t13273: F, t2240: F, t2301: F, t2309: F, t3601: F, t3665: F, t3674: F, t3678: F, t623: F, t650: F, t8614: F) -> F {
    let t13276 = t3677 * t7742;
    let t13279 = t2305 * t920;
    let t13289 = t623 * t13256 / F::cast_from(2.0_f64) + t623 * t13260 / F::cast_from(4.0_f64) + t623 * t13263 / F::cast_from(4.0_f64) + t2240 * t3665 / F::cast_from(2.0_f64) + t623 * t13268 / F::cast_from(4.0_f64) + t8614 * t1080 / F::cast_from(4.0_f64) + t13273 * t650 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t623 * t13276 + t623 * t13279 / F::cast_from(4.0_f64) + t3601 * t2301 / F::cast_from(4.0_f64) + t3601 * t2309 / F::cast_from(2.0_f64) + t2240 * t3674 / F::cast_from(2.0_f64) + t2240 * t3678;
    t13289
}
