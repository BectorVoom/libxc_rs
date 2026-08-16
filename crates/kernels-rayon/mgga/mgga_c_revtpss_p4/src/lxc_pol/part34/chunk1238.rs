//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1238/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1238(t11108: f64, t7840: f64, t1711: f64, t2411: f64, t10309: f64, t1470: f64, t28126: f64, t60224: f64, t6957: f64, t1513: f64, t94975: f64, t530: f64, t7933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t100806 = t7840 * t11108;
    let t100987 = t2411 * t1711;
    let t101252 = t10309 * t1470;
    let t101333 = t10309 * t28126;
    let t101342 = t60224 * t6957;
    let t101451 = t94975 * t1513;
    let t101473 = t530 * t7933;
    (t100806, t100987, t101252, t101333, t101342, t101451, t101473)
}
