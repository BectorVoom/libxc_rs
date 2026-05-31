//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 927/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk927<F: Float>(t13828: F, t13831: F, t13928: F, t14033: F, t14109: F, t14146: F, t14255: F, t14264: F, t14288: F, t14295: F, t14365: F, t247: F) -> F {
    let t14366 = -t14295 * t247 + F::cast_from(2.0_f64) * t13828 - F::cast_from(4.0_f64) * t13831 + F::cast_from(4.0_f64) * t13928 - F::cast_from(2.0_f64) * t14033 + F::cast_from(8.0_f64) * t14109 - F::cast_from(12.0_f64) * t14146 + F::cast_from(4.0_f64) * t14255 + F::cast_from(8.0_f64) * t14264 - F::cast_from(4.0_f64) * t14288 + t14365;
    t14366
}
