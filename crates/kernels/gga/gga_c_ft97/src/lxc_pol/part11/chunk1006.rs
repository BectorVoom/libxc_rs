//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1006/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1006<F: Float>(t1637: F, t2135: F, t89: F, t13208: F, t1651: F, t167: F, t1901: F, t2075: F, t2142: F, t2157: F, t2185: F, t2230: F, t38053: F, t38057: F, t39664: F, t41002: F, t41019: F, t446: F, t569: F, t574: F, t605: F, t609: F, t616: F, t7973: F, t9007: F, t9304: F, t9316: F) -> F {
    let t41040 = t89 * t1637 * t2135;
    let t41045 = -F::cast_from(80.0_f64) / F::cast_from(243.0_f64) * t446 * t41002 * t167 * t38053 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t569 * t2230 * t1651 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t446 * t569 * t616 * t7973 - t446 * t569 * t167 * t38057 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t41019 + F::cast_from(2.0_f64) * t446 * t574 * t605 * t2075 * t2157 + F::cast_from(4.0_f64) * t446 * t574 * t2142 * t9316 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t605 * t9007 * t609 - F::cast_from(8.0_f64) * t446 * t2185 * t2142 * t9304 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t41040 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t1901 * t13208 * t39664;
    t41045
}
