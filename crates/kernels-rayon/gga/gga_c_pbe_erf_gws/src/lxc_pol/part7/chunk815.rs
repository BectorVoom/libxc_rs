//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 815/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk815(t6495: f64, t6511: f64, t6522: f64, t6528: f64, t6532: f64, t6537: f64, t6540: f64, t6544: f64, t6565: f64, t6572: f64, t6597: f64, t6604: f64, t6607: f64, t6614: f64) -> f64 {
    let t6735 = t6495 - t6511 - t6522 - t6528 + t6532 + t6537 - t6540 - t6544 + t6565 + t6572 - t6597 - t6604 + t6607 + t6614;
    t6735
}
