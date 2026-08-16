//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 540/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk540(t220: f64, t34: f64, t2735: f64, t616: f64, t1031: f64, t202: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t2736 = t220 * t34;
    let t2737 = t2735 * t2736;
    let t2739 = 4.0_f64 / 15.0_f64 * t616 * t2737;
    let t2740 = t202 * t1031;
    let t2741 = t2740 * t184;
    (t2736, t2737, t2739, t2740, t2741)
}
