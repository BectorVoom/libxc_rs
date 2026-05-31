//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 751/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk751<F: Float>(t10153: F, t766: F, t242: F, t192: F, t7514: F, t265: F, t9708: F, t10090: F, t10094: F, t10123: F, t10126: F, t10128: F, t10131: F, t10134: F, t10137: F, t10140: F, t10143: F, t10146: F, t10148: F, t10151: F, t1901: F, t446: F) -> (F, F, F, F, F) {
    let t10154 = t10153 * t766;
    let t10155 = t242 * t10154;
    let t10157 = t192 * t7514;
    let t10159 = t10157 * t265 * t9708;
    let t10162 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10090 + t1901 * t10094 / F::cast_from(3.0_f64) - t446 * t10123 / F::cast_from(3.0_f64) + t10126 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t10128 - t446 * t10131 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t10134 - t446 * t10137 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10140 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t10143 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10146 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10148 - t446 * t10151 - t446 * t10155 - F::cast_from(2.0_f64) * t446 * t10159;
    (t10154, t10155, t10157, t10159, t10162)
}
