//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 525/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk525(t5456: f64, t89: f64, t1458: f64, t1774: f64, t1453: f64, t2331: f64, t1444: f64, t2341: f64, t5396: f64, t95: f64, t1419: f64, t1449: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5457 = t89 * t5456;
    let t5460 = t1774 * t1458;
    let t5464 = t1453 * t1453;
    let t5465 = t2331 * t5464;
    let t5468 = t1444 * t1444;
    let t5469 = t2341 * t5468;
    let t5472 = t95 * t5396;
    let t5475 = tau1 * t1419;
    let t5480 = t1449 * t1449;
    (t5457, t5460, t5464, t5465, t5468, t5469, t5472, t5475, t5480)
}
