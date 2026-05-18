//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 364/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk364<F: Float>(t156: F, t472: F, t1447: F, t1220: F, t1267: F, t1271: F, t1278: F, t1288: F, t1296: F, t1335: F, t1338: F, t1440: F, t1442: F, t1446: F) -> (F, F, F) {
    let t1448 = t156 * t472;
    let t1449 = t1447 * t1448;
    let t1450 = F::new(0.10843580882781524214e-1) * t1449;
    let t1451 = t1220 - t1271 - t1278 + t1335 + t1338 - t1440 - t1267 + t1442 + t1288 + t1296 + t1446 + t1450;
    (t1448, t1450, t1451)
}
