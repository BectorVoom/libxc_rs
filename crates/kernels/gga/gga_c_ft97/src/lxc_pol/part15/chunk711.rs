//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 711/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk711<F: Float>(t11949: F, t20105: F, t20109: F, t20119: F, t20126: F, t20132: F, t20139: F, t20143: F, t20147: F, t20154: F, t20331: F, t20390: F, t8455: F) -> F {
    let t20460 = t20147 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t20154 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t20132 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t20139 + t20143 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t20105 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t20109 - t11949 - t8455 + t20331 / F::cast_from(8.0_f64) + t20390 / F::cast_from(6.0_f64) + F::cast_from(2.0_f64) * t20119 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t20126;
    t20460
}
