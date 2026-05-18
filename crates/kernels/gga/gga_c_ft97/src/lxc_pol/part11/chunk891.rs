//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 891/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk891<F: Float>(t1555: F, t37357: F, t7764: F, t89: F, t37422: F, t37424: F, t37427: F, t37433: F, t38257: F, t38261: F, t38266: F, t38271: F, t38275: F, t38279: F, t38281: F, t38285: F, t38288: F) -> (F, F) {
    let t38292 = t89 * t1555 * t7764 * t37357;
    let t38294 = t37422 + F::new(2.0) / F::new(9.0) * t37424 + F::new(4.0) / F::new(3.0) * t37427 + F::new(4.0) * t37433 - t38257 / F::new(6.0) - t38261 + F::new(4.0) / F::new(3.0) * t38266 - F::new(4.0) / F::new(9.0) * t38271 - F::new(8.0) / F::new(9.0) * t38275 - F::new(2.0) / F::new(3.0) * t38279 - F::new(2.0) / F::new(9.0) * t38281 - F::new(2.0) / F::new(3.0) * t38285 + F::new(2.0) / F::new(27.0) * t38288 - F::new(4.0) / F::new(3.0) * t38292;
    (t38292, t38294)
}
