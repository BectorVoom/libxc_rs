//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1132/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1132(t14011: f64, t3237: f64, t3120: f64, t4023: f64, t14031: f64, t3228: f64, t14069: f64, t3123: f64, t367: f64, t6238: f64, t899: f64) -> (f64, f64, f64, f64, f64) {
    let t14489 = t14011 * t3237;
    let t14491 = t3120 * t4023;
    let t14493 = t14031 * t3228;
    let t14495 = t3123 * t14069;
    let t14498 = t899 * t6238 * t367;
    (t14489, t14491, t14493, t14495, t14498)
}
