//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 426/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk426(t2347: f64, t312: f64, t1250: f64, t1882: f64, t1234: f64, t2755: f64, t1228: f64, t1775: f64, t2: f64, t2766: f64, t848: f64, t1232: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4140 = t312 * t2347;
    let t4156 = t1882 * t1250;
    let t4191 = t2755 * t1234;
    let t4197 = t1775 * t1228;
    let t4199 = t2766 * t2;
    let t4206 = t848 * t2;
    let t4213 = t458 * t1232;
    (t4140, t4156, t4191, t4197, t4199, t4206, t4213)
}
