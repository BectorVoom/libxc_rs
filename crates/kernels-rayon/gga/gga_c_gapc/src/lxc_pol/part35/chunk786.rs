//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 786/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk786(t197: f64, t6055: f64, t1022: f64, t1979: f64, t3096: f64, t3094: f64, t1932: f64, t1936: f64, t3036: f64, t1894: f64, t1927: f64, t646: f64) -> (f64, f64, f64, f64) {
    let t9213 = t197 * t6055;
    let t9214 = t1022 * t9213;
    let t9216 = t3096 * t1979;
    let t9217 = t3094 * t9216;
    let t9219 = t1932 * t1936;
    let t9220 = t9219 * t3036;
    let t9222 = t1927 * t1894;
    let t9223 = t646 * t9222;
    (t9214, t9217, t9220, t9223)
}
