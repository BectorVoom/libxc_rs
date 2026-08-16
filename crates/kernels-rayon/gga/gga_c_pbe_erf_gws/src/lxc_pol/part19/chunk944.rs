//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 944/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk944(t3354: f64, t93: f64, t10636: f64, t10641: f64, t10646: f64, t108: f64, t1351: f64, t2538: f64, t2544: f64, t418: f64, t422: f64, t726: f64, t728: f64, t9788: f64, t9801: f64) -> f64 {
    let t10651 = t93 * t3354;
    let t10657 = (40.0_f64 / 27.0_f64 * t10636 * t418 + 80.0_f64 / 9.0_f64 * t2538 * t1351 + 20.0_f64 / 9.0_f64 * t10641 * t418 + 4.0_f64 / 3.0_f64 * t726 * t9788 + 40.0_f64 / 27.0_f64 * t10646 * t422 - 80.0_f64 / 9.0_f64 * t2544 * t1351 + 20.0_f64 / 9.0_f64 * t10651 * t422 + 4.0_f64 / 3.0_f64 * t728 * t9801) * t108;
    t10657
}
