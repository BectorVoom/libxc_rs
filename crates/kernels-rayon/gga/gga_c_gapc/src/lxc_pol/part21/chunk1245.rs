//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1245/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1245(t11258: f64, t3946: f64, t514: f64, t1005: f64, t13736: f64, t3639: f64, t4885: f64, t11273: f64, t8451: f64, t25526: f64, t3643: f64, t3646: f64) -> (f64, f64, f64, f64) {
    let t35400 = t514 * t3946 * t11258;
    let t35404 = t1005 * t13736 * t3639 * t4885;
    let t35406 = t8451 * t11273;
    let t35409 = t3643 * t25526 * t3646;
    (t35400, t35404, t35406, t35409)
}
