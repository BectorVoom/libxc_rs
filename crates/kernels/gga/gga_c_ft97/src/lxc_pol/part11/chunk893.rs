//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 893/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk893<F: Float>(t11262: F, t1526: F, t7713: F, t7763: F, t81: F, t11269: F, t1527: F, t1528: F, t3088: F, t38310: F, t38313: F, t38316: F, t7734: F, t7745: F, t7765: F, t7794: F, t7811: F, t7815: F, t8199: F) -> F {
    let t38319 = t1526 * t11262 * t7713;
    let t38327 = t81 * t7763;
    let t38339 = t38310 / F::new(18.0) - t38313 / F::new(6.0) - t38316 / F::new(12.0) - t38319 / F::new(9.0) - t1526 * t1527 * t7811 / F::new(4.0) - t1526 * t3088 * t7794 / F::new(3.0) - F::new(7.0) / F::new(27.0) * t1526 * t11269 * t38327 * t7765 - t1526 * t1527 * t7815 / F::new(4.0) - t1526 * t1527 * t1528 * t7745 / F::new(12.0) + t7734 + t8199;
    t38339
}
