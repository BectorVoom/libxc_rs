//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1288/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1288(t25526: f64, t3643: f64, t3646: f64, t11270: f64, t11273: f64, t25530: f64, t11262: f64, t8562: f64, t11235: f64, t15355: f64, t15358: f64, t3650: f64) -> (f64, f64, f64, f64) {
    let t35409 = t3643 * t25526 * t3646;
    let t35412 = t11270 * t25530 * t11273;
    let t35415 = t8562 * t11262;
    let t35419 = t3650 * t15355 * t11235 * t15358;
    (t35409, t35412, t35415, t35419)
}
