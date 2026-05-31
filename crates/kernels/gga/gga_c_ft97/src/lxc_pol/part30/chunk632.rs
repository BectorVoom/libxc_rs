//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 632/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk632<F: Float>(t10007: F, t28187: F, t1882: F, t6927: F, t11593: F, t1901: F, t24590: F, t24592: F, t28150: F, t28154: F, t28158: F, t28163: F, t28167: F, t28171: F, t28175: F, t28178: F, t28181: F, t28184: F, t446: F) -> F {
    let t28188 = t10007 * t28187;
    let t28191 = t1882 * t6927;
    let t28193 = t1901 * t28150 / F::cast_from(9.0_f64) + t1901 * t28154 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11593 * t28158 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t24590 - t24592 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t28163 - t446 * t28167 / F::cast_from(3.0_f64) - t446 * t28171 / F::cast_from(3.0_f64) - t446 * t28175 / F::cast_from(3.0_f64) - t446 * t28178 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t28181 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t28184 - t1901 * t28188 / F::cast_from(9.0_f64) + t28191 / F::cast_from(9.0_f64);
    t28193
}
