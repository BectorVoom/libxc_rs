//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2935/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2935<F: Float>(t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60189: F) -> F {
    let t61138 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t60163 + t60166 / F::cast_from(6.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t60168 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t60171 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t60173 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t48155 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t48157 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t48159 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t48161 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t48163 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t48165 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t48167 - F::cast_from(4.0_f64) * t60189;
    t61138
}
