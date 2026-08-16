//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1128/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1128(t1882: f64, t277: f64, t9959: f64, t11954: f64, t2981: f64, t876: f64, t1: f64, t1736: f64, t2206: f64, t311: f64, t3383: f64, t8675: f64) -> (f64, f64, f64, f64) {
    let t33998 = t277 * t1882 * t9959;
    let t34001 = t11954 * t2981 * t876;
    let t34005 = t311 * t2206 * t1736 * t1;
    let t34007 = t34005 * t8675 * t3383;
    (t33998, t34001, t34005, t34007)
}
