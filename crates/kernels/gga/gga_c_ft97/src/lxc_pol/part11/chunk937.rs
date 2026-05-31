//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 937/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk937<F: Float>(t8675: F, t8686: F, t8672: F, t1647: F, t1651: F, t2258: F, t2265: F, t2266: F, t2282: F, t2294: F, t37357: F, t379: F, t39439: F, t39441: F, t39449: F, t39451: F, t39453: F, t39455: F, t631: F, t8634: F, t8680: F, t8709: F) -> F {
    let t39457 = t8675 * t8686;
    let t39471 = t8675 * t8672;
    let t39481 = F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t39439 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t39441 + F::cast_from(2.0_f64) * t631 * t2258 * t8634 * t37357 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t39449 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t39451 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t39453 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t39455 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t39457 - F::cast_from(12.0_f64) * t2265 * t8680 * t1647 * t2282 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2265 * t2266 * t379 * t8709 - F::cast_from(2.0_f64) * t2265 * t2266 * t1651 * t2294 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t39471 + F::cast_from(4.0_f64) * t2265 * t2266 * t1647 * t2294 + F::cast_from(6.0_f64) * t2265 * t8680 * t1651 * t2282;
    t39481
}
