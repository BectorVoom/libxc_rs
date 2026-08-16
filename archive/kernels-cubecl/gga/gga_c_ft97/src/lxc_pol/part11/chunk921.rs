//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 921/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk921<F: Float>(t1827: F, t8232: F, t1882: F, t8468: F, t1637: F, t1843: F, t89: F, t11587: F, t12020: F, t1825: F, t1871: F, t1901: F, t3193: F, t38300: F, t38304: F, t38379: F, t38648: F, t38665: F, t38930: F, t446: F, t83: F, t8377: F, t8539: F) -> F {
    let t38983 = t8232 * t1827;
    let t38988 = t1882 * t8468;
    let t38991 = t89 * t1637 * t1843;
    let t39000 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t3193 * t12020 * t38930 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1901 * t11587 * t8377 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t38304 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t83 * t38300 + F::cast_from(2.0_f64) * t446 * t83 * t38648 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t38983 - F::cast_from(2.0_f64) * t446 * t83 * t38379 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t38988 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t38991 + F::cast_from(4.0_f64) * t446 * t83 * t38665 - F::cast_from(8.0_f64) * t446 * t1871 * t1825 * t8539;
    t39000
}
