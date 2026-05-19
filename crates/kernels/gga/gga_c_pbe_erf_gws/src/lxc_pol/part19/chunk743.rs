//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 743/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk743<F: Float>(t13: F, t4508: F, t1275: F, t403: F, t1293: F, t14: F, t25: F, t2: F, t39: F, t784: F, t799: F, t1236: F) -> (F, F, F, F, F, F) {
    let t4509 = t13 * t4508;
    let t4510 = t1275 * t403;
    let t4511 = t4510 * t1293;
    let t4512 = t4509 * t4511;
    let t4513 = F::cast_from(0.96490945932906628932e2_f64) * t4512;
    let t4516 = F::new(1.0) / t14 / t25 / F::new(4.0);
    let t4517 = t4516 * t2;
    let t4518 = t4517 * t39;
    let t4520 = t799 * t784;
    let t4521 = t1236 * t4520;
    (t4510, t4513, t4516, t4518, t4520, t4521)
}
