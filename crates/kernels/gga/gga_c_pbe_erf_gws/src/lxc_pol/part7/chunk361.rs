//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 361/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk361<F: Float>(t1425: F, t40: F, t414: F, t428: F, t461: F, t409: F, t1: F, t427: F, t467: F, t1328: F, t1330: F, t1394: F, t1398: F, t1401: F, t1424: F) -> (F, F, F, F, F, F, F) {
    let t1426 = t40 * t1425;
    let t1427 = F::new(2.0) * t1426;
    let t1428 = t414 * t428;
    let t1429 = F::new(8.0) * t1428;
    let t1430 = t414 * t461;
    let t1431 = F::new(8.0) * t1430;
    let t1432 = t409 * t428;
    let t1433 = F::new(8.0) * t1432;
    let t1434 = t427 * t1;
    let t1435 = t1434 * t467;
    let t1436 = F::new(0.36623110073506319882e-3) * t1435;
    let t1437 = -t1394 - t1398 - t1401 + t1328 + t1424 + t1427 - t1429 - t1431 + t1433 + t1330 - t1436;
    (t1427, t1429, t1431, t1433, t1434, t1436, t1437)
}
