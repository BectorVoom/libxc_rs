//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 925/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk925(t31169: f64, t3777: f64, t31172: f64, t12402: f64, t31170: f64, t5248: f64, t550: f64, t1336: f64, t1338: f64, t241: f64, t835: f64, t240: f64, t3787: f64) -> (f64, f64, f64, f64) {
    let t114002 = t3777 * t31169;
    let t114003 = t114002 * t31172;
    let t114007 = t31170 * t5248 * t12402 * t550;
    let t114011 = t1336 * t1338 * t835 * t241;
    let t114012 = t114011 * t31172;
    let t114016 = t1336 * t3787 * t240 * t241;
    (t114003, t114007, t114012, t114016)
}
