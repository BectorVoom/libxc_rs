//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 597/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk597<F: Float>(t3051: F, t94: F, t1771: F, t471: F, t24: F, t469: F, t8183: F, t1781: F, t7765: F, t463: F, t3134: F, t7789: F) -> (F, F, F, F, F, F) {
    let t8301 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t3051 * t94;
    let t8302 = t1771 * t471;
    let t8305 = t24 * t469 * t8183;
    let t8307 = t1781 * t7765;
    let t8308 = t463 * t8307;
    let t8311 = t3134 * t7789;
    (t8301, t8302, t8305, t8307, t8308, t8311)
}
