//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 786/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk786<F: Float>(t10279: F, t10282: F, t10259: F, t10265: F, t10269: F, t10273: F, t10391: F, t10552: F, t10553: F, t10555: F, t10624: F, t10634: F, t10636: F) -> F {
    let t10640 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t10279;
    let t10641 = t10282 / F::cast_from(9.0_f64);
    let t10642 = -t10391 / F::cast_from(3.0_f64) + t10552 - t10553 - F::cast_from(2.0_f64) * t10265 - t10555 + t10624 / F::cast_from(6.0_f64) + t10634 / F::cast_from(8.0_f64) - t10636 - t10259 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) * t10269 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t10273 - t10640 + t10641;
    t10642
}
