//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 951/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk951(t7438: f64, t7442: f64, t7447: f64, t7451: f64, t7456: f64, t7461: f64, t7466: f64, t7472: f64, t7474: f64, t7476: f64, t7478: f64, t7479: f64, t7480: f64, t7482: f64, t7489: f64, t7494: f64) -> f64 {
    let t8430 = t7438 - t7442 - t7447 + t7451 + t7456 - t7461 + t7466 - t7472 - t7474 - t7476 - t7478 + t7479 + t7480 + t7482 - t7489 + t7494;
    t8430
}
