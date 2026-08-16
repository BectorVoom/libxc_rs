//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1117/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1117(t2373: f64, t4424: f64, t6127: f64, t9296: f64, t829: f64, t830: f64, t4379: f64, t831: f64, t2370: f64, t4417: f64, t814: f64, t2379: f64, t4474: f64) -> (f64, f64, f64, f64, f64) {
    let t20049 = t4424 * t2373;
    let t20051 = t9296 * t6127;
    let t20053 = t829 * t830 * t20051;
    let t20056 = t831 * t4379;
    let t20058 = t2370 * t830 * t20056;
    let t20063 = t829 * t830 * t4417 * t814;
    let t20076 = t4474 * t2379;
    (t20049, t20053, t20058, t20063, t20076)
}
