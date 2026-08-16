//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 742/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk742(t1: f64, t2293: f64, t106: f64, t192: f64, t1407: f64, t2483: f64, t1406: f64, t2486: f64) -> (f64, f64, f64) {
    let t7005 = t2293 * t1;
    let t7006 = t7005 * t106;
    let t7007 = t7006 * t192;
    let t7012 = t1407 * t2483;
    let t7014 = t1406 * t2486;
    (t7007, t7012, t7014)
}
