//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 593/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk593<F: Float>(t8189: F, t2: F, t432: F, t1587: F, t1755: F, t464: F, t7745: F, t463: F, t7241: F, t24: F, t7751: F, t7760: F, t82: F) -> (F, F, F, F, F, F, F) {
    let t8260 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t8189;
    let t8261 = t2 * t432;
    let t8263 = t1587 * t8261 * t1755;
    let t8266 = t464 * t7745;
    let t8267 = t463 * t8266;
    let t8270 = t7241 * t2;
    let t8272 = t24 * t8270 * t7751;
    let t8275 = t7760 * t82;
    (t8260, t8261, t8263, t8266, t8267, t8272, t8275)
}
