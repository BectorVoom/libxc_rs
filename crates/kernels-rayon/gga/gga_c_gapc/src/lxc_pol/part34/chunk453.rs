//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 453/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk453(t2199: f64, t2259: f64, t2422: f64, t2460: f64, t880: f64, t883: f64, t337: f64, t882: f64) -> (f64, f64, f64) {
    let t2462 = t2199 + t2259 + t2422 + t2460;
    let t2464 = t880 * t883;
    let t2468 = 1.0_f64 / t882 / t337;
    (t2462, t2464, t2468)
}
