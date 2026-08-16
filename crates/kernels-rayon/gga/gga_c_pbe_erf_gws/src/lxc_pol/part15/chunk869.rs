//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 869/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk869(t1036: f64, t5463: f64, t639: f64, t1802: f64, t1804: f64, t995: f64, t1885: f64, t1820: f64, t188: f64, t331: f64, t34: f64, t597: f64) -> (f64, f64, f64, f64) {
    let t7459 = t5463 * t1036;
    let t7460 = t639 * t7459;
    let t7461 = 8.0_f64 / 405.0_f64 * t7460;
    let t7463 = t1802 * t995 * t1804;
    let t7464 = t1885 * t7463;
    let t7466 = 8.0_f64 / 15.0_f64 * t1820 * t7464;
    let t7467 = t331 * t188;
    let t7468 = t597 * t34;
    (t7461, t7466, t7467, t7468)
}
