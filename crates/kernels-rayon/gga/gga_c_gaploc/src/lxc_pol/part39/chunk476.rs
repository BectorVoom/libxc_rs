//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 476/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk476(t493: f64, t6519: f64, t6509: f64, t1339: f64, t1422: f64, t4389: f64, t544: f64, t1: f64, t6514: f64, t584: f64, t6715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6750 = t493 * t6519;
    let t6763 = t493 * t6509;
    let t6767 = t1339 * t6509;
    let t6823 = t4389 * t1422;
    let t6824 = t544 * t6823;
    let t6851 = t6514 * t1;
    let t6914 = t584 * t6715;
    (t6750, t6763, t6767, t6824, t6851, t6914)
}
