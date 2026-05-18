//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 589/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk589<F: Float>(t4422: F, t824: F, t2200: F, t329: F, t340: F, t2306: F, t2365: F, t1327: F, t409: F, t1285: F, t1291: F, t1293: F, t403: F) -> (F, F, F, F, F, F) {
    let t4423 = t824 * t4422;
    let t4442 = t329 * t2200 * t340;
    let t4473 = t2306 * t2365;
    let t4498 = t409 * t1327;
    let t4499 = F::new(12.0) * t4498;
    let t4502 = t1291 * t1285 * t1293 * t403;
    (t4423, t4442, t4473, t4498, t4499, t4502)
}
