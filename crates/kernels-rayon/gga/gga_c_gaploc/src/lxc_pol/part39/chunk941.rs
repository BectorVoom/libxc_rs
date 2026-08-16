//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 941/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk941(t42187: f64, t26984: f64, t9294: f64, t1424: f64, t2875: f64, t544: f64, t9065: f64, t20368: f64, t41596: f64, t20367: f64, t4820: f64, t12953: f64, t31054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42188 = 0.42603251059911944084e0_f64 * t42187;
    let t42189 = t26984 * t9294;
    let t42190 = 0.89376224879626066675e-1_f64 * t42189;
    let t42194 = 0.39722766613167140743e-1_f64 * t544 * t9065 * t2875 * t1424;
    let t42195 = t20368 * t41596;
    let t42198 = 0.23833659967900284446e0_f64 * t20367 * t4820 * t42195;
    let t42199 = t31054 * t12953;
    (t42188, t42190, t42194, t42195, t42198, t42199)
}
