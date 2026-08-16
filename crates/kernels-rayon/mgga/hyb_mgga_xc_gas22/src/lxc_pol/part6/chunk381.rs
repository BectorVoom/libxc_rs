//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 381/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk381(t1399: f64, t950: f64, t957: f64, t1392: f64, t343: f64, t238: f64, t242: f64, t1394: f64, t955: f64, t962: f64) -> (f64, f64, f64, f64, f64) {
    let t1400 = t950 * t1399;
    let t1403 = t957 * t1399;
    let t1405 = t343 * t1392;
    let t1407 = t238 * t242 * t1405;
    let t1409 = 0.1898925e1_f64 * t1400 - t955 + 0.8969e0_f64 * t1394 + 0.3071625e0_f64 * t1403 - t962 + 0.24647e0_f64 * t1407;
    (t1400, t1403, t1405, t1407, t1409)
}
