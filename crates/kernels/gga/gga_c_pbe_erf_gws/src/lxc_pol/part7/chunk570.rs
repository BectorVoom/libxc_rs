//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 570/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk570<F: Float>(t4344: F, t116: F, t784: F, t799: F, t798: F, t259: F, t47: F, t1403: F, t418: F) -> (F, F, F, F, F, F) {
    let t4345 = F::cast_from(0.1232289865202e1_f64) * t4344;
    let t4347 = t799 * t784 * t116;
    let t4348 = t798 * t4347;
    let t4349 = F::cast_from(0.18256146151140740741e1_f64) * t4348;
    let t4351 = F::new(1.0) / t47 / t259;
    let t4352 = t1403 * t418;
    (t4345, t4347, t4348, t4349, t4351, t4352)
}
