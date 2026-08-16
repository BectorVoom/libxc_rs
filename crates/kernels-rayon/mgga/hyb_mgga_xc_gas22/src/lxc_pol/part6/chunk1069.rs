//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1069/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1069(t3925: f64, t6299: f64, t3171: f64, t675: f64, t3: f64, t3172: f64, t1890: f64, t4006: f64, t2054: f64, t3177: f64, t3178: f64, t4010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10293 = t6299 * t3925;
    let t10295 = t3171 * t10293 * t675;
    let t10299 = t3171 * t3172 * t3;
    let t10302 = t1890 * t4006;
    let t10304 = t2054 * t3925;
    let t10306 = t3177 * t10304 * t675;
    let t10310 = t3177 * t3178 * t3;
    let t10313 = t1890 * t4010;
    (t10293, t10295, t10299, t10302, t10304, t10306, t10310, t10313)
}
