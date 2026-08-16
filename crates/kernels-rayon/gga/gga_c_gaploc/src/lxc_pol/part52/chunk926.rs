//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 926/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk926(t46641: f64, t46103: f64, t6963: f64, t6964: f64, t13465: f64, t1407: f64, t10430: f64, t10608: f64, t9272: f64, t11400: f64, t1424: f64, t2299: f64, t544: f64) -> (f64, f64, f64, f64, f64) {
    let t46642 = 0.42603251059911944084e-1_f64 * t46641;
    let t46645 = 0.71500979903700853338e0_f64 * t6963 * t6964 * t46103;
    let t46646 = t1407 * t13465;
    let t46653 = t9272 * t10608 * t10430;
    let t46654 = 0.11502877786176224903e1_f64 * t46653;
    let t46658 = 0.39722766613167140743e-1_f64 * t544 * t2299 * t11400 * t1424;
    (t46642, t46645, t46646, t46654, t46658)
}
