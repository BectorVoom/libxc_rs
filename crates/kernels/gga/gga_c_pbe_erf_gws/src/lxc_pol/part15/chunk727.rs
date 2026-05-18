//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 727/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk727<F: Float>(t4502: F, t1274: F, t1285: F, t405: F, t1289: F, t27: F, t13: F, t1275: F, t403: F, t1293: F, t14: F, t25: F) -> (F, F, F, F, F) {
    let t4503 = F::new(0.48245472966453314466e2) * t4502;
    let t4505 = t1274 * t405 * t1285;
    let t4506 = F::new(6.0) * t4505;
    let t4508 = F::new(1.0) / t1289 / t27;
    let t4509 = t13 * t4508;
    let t4510 = t1275 * t403;
    let t4511 = t4510 * t1293;
    let t4512 = t4509 * t4511;
    let t4513 = F::new(0.96490945932906628932e2) * t4512;
    let t4516 = F::new(1.0) / t14 / t25 / F::new(4.0);
    (t4503, t4506, t4510, t4513, t4516)
}
