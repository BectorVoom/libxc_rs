//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 815/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk815(t9709: f64, t9710: f64, t8986: f64, t961: f64, t7967: f64, t1072: f64, t1074: f64, t2387: f64, t1069: f64, t2489: f64, t102: f64, t818: f64) -> (f64, f64, f64, f64, f64) {
    let t9711 = t9709 * t9710;
    let t9713 = t8986 * t961;
    let t9714 = t7967 * t9713;
    let t9717 = t2387 * t1072 * t1074;
    let t9719 = t1069 * t2489;
    let t9721 = t102 * t818;
    (t9711, t9714, t9717, t9719, t9721)
}
