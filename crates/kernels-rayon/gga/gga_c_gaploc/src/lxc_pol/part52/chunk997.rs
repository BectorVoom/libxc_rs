//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 997/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk997(t447: f64, t49921: f64, t1445: f64, t46240: f64, t46244: f64, t46246: f64, t46250: f64, t46252: f64, t46257: f64, t46261: f64, t46264: f64, t46267: f64, t46271: f64, t46275: f64, t46283: f64, t46287: f64, t46289: f64, t46291: f64, t46294: f64, t46297: f64, t49874: f64, t49878: f64, t574: f64, t597: f64, t6716: f64, t6717: f64) -> (f64, f64) {
    let t50596 = t49921 * t447;
    let t50606 = 0.51123901271894332901e0_f64 * t46240 - t46244 - t46246 - t46250 - t46252 - t46257 - t46261 + 0.13803453343411469884e2_f64 * t6716 * t6717 * t50596 + t46264 - t46267 + t46271 + t46275 + t46283 - 0.46011511144704899612e1_f64 * t574 * t1445 * t49878 + 0.11502877786176224903e2_f64 * t597 * t1445 * t49874 - t46287 - t46289 - t46291 + t46294 + t46297;
    (t50596, t50606)
}
