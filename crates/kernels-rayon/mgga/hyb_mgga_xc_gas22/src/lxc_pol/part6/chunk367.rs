//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 367/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk367(t1336: f64, t789: f64, t796: f64, t1329: f64, t226: f64, t238: f64, t242: f64, t1331: f64, t794: f64, t804: f64) -> (f64, f64, f64, f64, f64) {
    let t1337 = t789 * t1336;
    let t1340 = t796 * t1336;
    let t1342 = t226 * t1329;
    let t1344 = t238 * t242 * t1342;
    let t1346 = 0.1898925e1_f64 * t1337 - t794 + 0.8969e0_f64 * t1331 + 0.3071625e0_f64 * t1340 - t804 + 0.24647e0_f64 * t1344;
    (t1337, t1340, t1342, t1344, t1346)
}
