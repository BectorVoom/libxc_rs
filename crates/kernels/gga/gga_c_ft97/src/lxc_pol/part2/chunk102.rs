//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 102/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk102<F: Float>(t274: F, t278: F, t220: F) -> (F, F, F, F, F) {
    let t280 = F::cast_from(0.1247511874e1_f64) - F::cast_from(0.859614445e0_f64) * t274 + F::cast_from(0.812904345e0_f64) * t278;
    let t281 = t280 * t280;
    let t282 = F::cast_from(0.56633563016285904186e-1_f64) * t220;
    let t283 = F::new(1.0) + t282;
    let t284 = t283 * t283;
    (t280, t281, t282, t283, t284)
}
