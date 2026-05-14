//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1098/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1098<F: Float>(t50: F, t13020: F, t16241: F, t1940: F, t22034: F, t3339: F, t4573: F, t55917: F, t55922: F, t55927: F, t611: F, t55916: F, zeta_threshold: F) -> (F,) {
    let t51 = t50 <= zeta_threshold;
    let t55931 = piecewise3(t51, 0.0, -56.0 / 81.0 * t22034 * t55917 + 16.0 / 9.0 * t13020 * t4573 - 2.0 / 3.0 * t1940 * t55922 - 8.0 / 9.0 * t3339 * t16241 + 2.0 / 3.0 * t611 * t55927);
    let t55933 = t55916 / 2.0 + t55931 / 2.0;
    (t55933,)
}
