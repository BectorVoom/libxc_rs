//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 718/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk718<F: Float>(t4347: F, t798: F, t259: F, t47: F, t34: F, t39: F, t261: F, t52: F, t56: F, t825: F) -> (F, F, F, F, F) {
    let t4348 = t798 * t4347;
    let t4349 = F::new(0.18256146151140740741e1) * t4348;
    let t4351 = F::new(1.0) / t47 / t259;
    let t4358 = t34 * t39;
    let t4366 = F::new(1.0) / t52 / t261;
    let t4383 = t825 * t56;
    (t4349, t4351, t4358, t4366, t4383)
}
