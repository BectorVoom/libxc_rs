//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1102/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1102(t14001: f64, t3960: f64, t2087: f64, t4023: f64, t3969: f64, t915: f64, t2276: f64) -> (f64, f64, f64, f64, f64) {
    let t14002 = t14001 * t3960;
    let t14003 = 7.0_f64 / 72.0_f64 * t14002;
    let t14004 = t2087 * t4023;
    let t14006 = t3969 * t915;
    let t14007 = t2276 * t14006;
    (t14002, t14003, t14004, t14006, t14007)
}
