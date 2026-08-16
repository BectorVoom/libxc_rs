//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 777/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk777(t20700: f64, t6710: f64, t9438: f64, t20551: f64, t6914: f64, t20696: f64, t2476: f64, t20561: f64, t2487: f64, t12444: f64, t2464: f64, t587: f64) -> (f64, f64, f64, f64, f64) {
    let t40372 = t6710 * t9438 * t20700;
    let t40377 = t6914 * t9438 * t20551;
    let t40449 = t2476 * t9438 * t20696;
    let t40452 = t2487 * t9438 * t20561;
    let t40517 = t587 * t2464 * t12444;
    (t40372, t40377, t40449, t40452, t40517)
}
