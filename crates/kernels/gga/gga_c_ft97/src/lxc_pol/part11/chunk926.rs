//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 926/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk926<F: Float>(t1882: F, t8396: F, t488: F, t8326: F, t8583: F, t8574: F, t110: F, t1643: F, t1820: F, t1866: F, t1876: F, t1901: F, t1904: F, t358: F, t379: F, t38086: F, t38654: F, t39150: F, t39154: F, t446: F, t447: F, t499: F, t7959: F, t8219: F, t83: F, t8553: F, t8557: F) -> F {
    let t39161 = t1882 * t8396;
    let t39167 = t8326 * t488;
    let t39183 = t1882 * t8583;
    let t39185 = t1882 * t8574;
    let t39187 = -F::new(8.0) / F::new(3.0) * t1901 * t39150 * t8219 + F::new(8.0) / F::new(9.0) * t39154 - F::new(4.0) / F::new(3.0) * t1901 * t8557 * t1820 * t358 * t1904 + F::new(4.0) / F::new(3.0) * t39161 - F::new(4.0) / F::new(3.0) * t1901 * t8557 * t8553 * t379 - F::new(8.0) / F::new(9.0) * t1901 * t39167 * t1876 * t1643 + F::new(8.0) * t446 * t83 * t38654 + F::new(8.0) / F::new(3.0) * t446 * t447 * t110 * t38086 + F::new(16.0) / F::new(9.0) * t446 * t1866 * t499 * t7959 + F::new(4.0) / F::new(9.0) * t39183 + F::new(4.0) / F::new(27.0) * t39185;
    t39187
}
