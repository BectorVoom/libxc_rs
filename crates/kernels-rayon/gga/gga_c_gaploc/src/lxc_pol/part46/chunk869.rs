//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 869/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk869(t20671: f64, t31047: f64, t34814: f64, t26984: f64, t9294: f64, t1424: f64, t2875: f64, t544: f64, t9065: f64, t20368: f64, t41596: f64, t20367: f64, t4820: f64) -> (f64, f64, f64, f64, f64) {
    let t42187 = t31047 * t20671 * t34814;
    let t42188 = 0.42603251059911944084e0_f64 * t42187;
    let t42189 = t26984 * t9294;
    let t42190 = 0.89376224879626066675e-1_f64 * t42189;
    let t42194 = 0.39722766613167140743e-1_f64 * t544 * t9065 * t2875 * t1424;
    let t42195 = t20368 * t41596;
    let t42198 = 0.23833659967900284446e0_f64 * t20367 * t4820 * t42195;
    (t42188, t42190, t42194, t42195, t42198)
}
