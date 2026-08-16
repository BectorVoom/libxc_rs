//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 340/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk340<F: Float>(t1327: F, t40: F, t409: F, t461: F, t37: F, t38: F) -> (F, F, F, F) {
    let t1328 = t40 * t1327;
    let t1329 = t409 * t461;
    let t1330 = F::cast_from(8.0_f64) * t1329;
    let t1331 = t38 * t37;
    let t1332 = F::cast_from(1.0_f64) / t1331;
    (t1328, t1330, t1331, t1332)
}
