//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 585/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk585(t3103: f64, t889: f64, t2636: f64, t787: f64, t1081: f64, t949: f64, t122: f64, t932: f64) -> (f64, f64, f64, f64, f64) {
    let t3396 = t889 * t3103;
    let t3397 = t2636 * t787;
    let t3398 = t3396 * t3397;
    let t3400 = t1081 * t949;
    let t3402 = t932 * t122;
    (t3396, t3397, t3398, t3400, t3402)
}
