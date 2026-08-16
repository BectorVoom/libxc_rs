//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1068/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1068<F: Float>(t2: F, t42123: F, t1775: F, t9913: F, t9928: F, t9910: F, t2493: F, t3910: F, t3917: F, t41464: F, t41490: F, t41827: F, t41833: F, t41880: F, t41884: F, t42105: F, t42107: F, t42110: F, t42117: F, t42119: F, t42121: F, t462: F, t9916: F) -> F {
    let t42124 = t42123 * t2;
    let t42131 = t1775 * t9913;
    let t42133 = t1775 * t9928;
    let t42141 = t1775 * t9910;
    let t42143 = -F::cast_from(8.0_f64) * t42105 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t42107 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t462 * t42110 * t41880 - F::cast_from(4.0_f64) * t462 * t2493 * t41833 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t42117 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t42119 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42121 + F::cast_from(8.0_f64) * t462 * t42124 * t41827 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t462 * t9916 * t41884 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42131 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t42133 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t462 * t3917 * t41490 + F::cast_from(8.0_f64) * t462 * t3910 * t41464 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t42141;
    t42143
}
