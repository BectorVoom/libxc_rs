//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1180/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1180<F: Float>(t1359: F, t6723: F, t1349: F, t138420: F, t1389: F, t139159: F, t139171: F, t139179: F, t147856: F, t148120: F, t148205: F, t149191: F, t1969: F, t26546: F, t28: F, t32686: F, t32876: F, t3408: F, t379: F, t5772: F, t5778: F, t6580: F, t6589: F, t7309: F, t9073: F, t925: F) -> F {
    let t149419 = t1359 * t6723;
    let t149432 = -F::cast_from(12.0_f64) * t147856 - t32686 * t6589 / F::cast_from(3.0_f64) + t139159 / F::cast_from(9.0_f64) + t5772 * t9073 * t138420 * t925 / F::cast_from(9.0_f64) - t7309 * t26546 / F::cast_from(3.0_f64) + t6580 * t32876 / F::cast_from(6.0_f64) - t139171 / F::cast_from(9.0_f64) - t139179 / F::cast_from(9.0_f64) - t5772 * t1969 * t149419 * t379 / F::cast_from(9.0_f64) + F::cast_from(8.0_f64) * t148205 + F::cast_from(8.0_f64) * t149191 + F::cast_from(8.0_f64) * t148120 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1349 * t28 * t5778 * t1389 * t3408;
    t149432
}
