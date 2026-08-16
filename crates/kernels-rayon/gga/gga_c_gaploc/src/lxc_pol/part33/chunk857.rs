//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 857/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk857(t723: f64, t8469: f64, t1445: f64, t1710: f64, t2958: f64, t2936: f64, t769: f64, t2089: f64, t2925: f64, t3009: f64, t1457: f64, t1022: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8470 = t8469 * t723;
    let t8471 = t1445 * t8470;
    let t8474 = t2958 * t1710;
    let t8475 = t1445 * t8474;
    let t8478 = t769 * t2936;
    let t8483 = t2089 * t2925;
    let t8484 = t8483 * t723;
    let t8485 = t1445 * t8484;
    let t8488 = t3009 * t1710;
    let t8489 = t1445 * t8488;
    let t8494 = t1457 * t8470;
    let t8497 = t1457 * t8474;
    let t8502 = t1022 * t723;
    (t8470, t8471, t8474, t8475, t8478, t8483, t8485, t8489, t8494, t8497, t8502)
}
