//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1424/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1424(t20168: f64, t31540: f64, t20158: f64, t31735: f64, t20172: f64, t2854: f64, t590: f64, t6519: f64, t2875: f64, t544: f64, t6514: f64, t1367: f64, t20901: f64) -> (f64, f64, f64, f64) {
    let t35172 = 0.51123901271894332902e1_f64 * t20168 * t31540;
    let t35174 = 0.2044956050875773316e1_f64 * t20158 * t31735;
    let t35178 = 0.30674340763136599742e1_f64 * t20172 * t2854 * t6519 * t590;
    let t35180 = t544 * t6514 * t2875;
    let t35183 = 0.55611873258433997041e0_f64 * t35180 * t20901 * t1367;
    (t35172, t35174, t35178, t35183)
}
