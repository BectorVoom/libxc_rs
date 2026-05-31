//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1188/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1188<F: Float>(t10580: F, t10603: F, t10613: F, t14961: F, t2766: F, t2771: F, t4206: F, t43834: F, t43913: F, t43918: F, t462: F, t70999: F, t848: F, t88253: F, t88257: F, t88261: F, t88273: F, t88606: F, t88612: F, t89779: F, t89822: F, t89826: F, t89870: F, t89877: F, t89881: F, t89885: F) -> F {
    let t90421 = F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t462 * t10580 * t88253 - F::cast_from(8.0_f64) * t462 * t2766 * t88273 + F::cast_from(8.0_f64) * t462 * t848 * t88257 + F::cast_from(2.0_f64) * t462 * t848 * t88261 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t70999 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t462 * t43918 * t89881 - F::cast_from(12.0_f64) * t462 * t4206 * t88606 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t462 * t14961 * t88612 - F::cast_from(4.0_f64) * t462 * t2771 * t89885 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t10613 * t89877 - F::cast_from(4.0_f64) * t462 * t10603 * t89822 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t462 * t2771 * t89826 + F::cast_from(8.0_f64) * t462 * t43913 * t89779 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t462 * t43834 * t89870;
    t90421
}
