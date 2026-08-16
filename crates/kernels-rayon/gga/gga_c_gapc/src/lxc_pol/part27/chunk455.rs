//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 455/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk455(t2493: f64, t617: f64, t2153: f64, t285: f64, t191: f64, t1936: f64, t320: f64, t291: f64, t481: f64, t297: f64, t875: f64, t941: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2494 = t617 * t2493;
    let t2497 = t2153 * t285;
    let t2498 = t2497 * t191;
    let t2501 = t320 * t1936;
    let t2502 = t481 * t291;
    let t2503 = t2502 * t297;
    let t2504 = t941 * t875;
    (t2494, t2497, t2498, t2501, t2502, t2503, t2504)
}
