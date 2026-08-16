//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1299/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1299(t3643: f64, t423: f64, t11203: f64, t8297: f64, t11204: f64, t25382: f64, t1006: f64, t125: f64, t1552: f64, t1954: f64, t200: f64, t11227: f64, t8291: f64) -> (f64, f64, f64, f64) {
    let t35491 = t3643 * t423;
    let t35493 = t35491 * t11203 * t8297;
    let t35495 = t11204 * t25382;
    let t35500 = t1006 * t125 * t1552 * t200 * t1954;
    let t35503 = t35491 * t11227 * t8291;
    (t35493, t35495, t35500, t35503)
}
