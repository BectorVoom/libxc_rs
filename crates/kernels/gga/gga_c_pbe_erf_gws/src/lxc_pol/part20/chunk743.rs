//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 743/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk743<F: Float>(t2242: F, t894: F, t1327: F, t409: F, t1285: F, t1291: F, t1293: F, t403: F, t1274: F, t405: F, t1289: F, t27: F) -> (F, F, F, F, F) {
    let t4487 = t2242 * t894;
    let t4498 = t409 * t1327;
    let t4502 = t1291 * t1285 * t1293 * t403;
    let t4503 = F::cast_from(0.48245472966453314466e2_f64) * t4502;
    let t4505 = t1274 * t405 * t1285;
    let t4506 = F::new(6.0) * t4505;
    let t4508 = F::new(1.0) / t1289 / t27;
    (t4487, t4498, t4503, t4506, t4508)
}
