//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1104/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1104<F: Float>(t10845: F, t10864: F, t2265: F, t2405: F, t2409: F, t2413: F, t2923: F, t2939: F, t2951: F, t43146: F, t43148: F, t43150: F, t43152: F, t43158: F, t43160: F, t43162: F, t43164: F, t43177: F, t904: F, t9578: F) -> F {
    let t43183 = -F::cast_from(2.0_f64) * t2265 * t2923 * t2413 * t2951 - F::cast_from(8.0_f64) * t43146 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t43148 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43150 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43152 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2265 * t10845 * t2405 * t2951 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43158 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t43160 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43162 + F::cast_from(2.0_f64) * t2265 * t43164 * t2405 * t2939 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2265 * t10845 * t9578 * t904 + F::cast_from(4.0_f64) * t2265 * t2923 * t2409 * t2951 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t43177 + F::cast_from(6.0_f64) * t2265 * t10864 * t2413 * t2939;
    t43183
}
