//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 576/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk576<F: Float>(t1: F, t959: F, t467: F, t1220: F, t1278: F, t1288: F, t1296: F, t1328: F, t1335: F, t1338: F, t1426: F, t1431: F, t1450: F, t2064: F, t2449: F, t2456: F, t2476: F) -> (F, F, F) {
    let t2506 = t959 * t1;
    let t2507 = t2506 * t467;
    let t2508 = F::cast_from(0.18311555036753159941e-3_f64) * t2507;
    let t2509 = t1220 + t1328 + t1335 - t1338 + t1426 - t2449 + t1450 - t1278 + t1288 + t1296 - t2456 + t2476 - t2064 - t2508 - t1431;
    (t2506, t2508, t2509)
}
