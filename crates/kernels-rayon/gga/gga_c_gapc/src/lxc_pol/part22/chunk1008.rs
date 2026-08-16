//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1008/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1008(t2456: f64, t3728: f64, t1062: f64, t10335: f64, t3643: f64, t3734: f64, t10286: f64, t11270: f64, t2923: f64, t7108: f64, t959: f64, t3225: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11640 = t3728 * t2456;
    let t11641 = t1062 * t11640;
    let t11643 = t3643 * t10335;
    let t11644 = t11643 * t3734;
    let t11646 = t11270 * t10286;
    let t11648 = t2923 * t959 * t7108;
    let t11649 = t11646 * t11648;
    let t11651 = t3225 * t3734;
    (t11640, t11641, t11644, t11648, t11649, t11651)
}
