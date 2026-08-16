//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1042/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1042(t178: f64, t8358: f64, t2364: f64, t2394: f64, t2886: f64, t980: f64, t6517: f64, t919: f64, t1227: f64, t2411: f64, t300: f64, t1235: f64, t297: f64, t46: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10043 = t8358 * t178;
    let t10044 = t2364 * t10043;
    let t10047 = t2394 * t10043;
    let t10063 = t980 * t2886;
    let t10121 = t6517 * t919;
    let t10212 = t2411 * t1227;
    let t10213 = t300 * t10212;
    let t10257 = t1235 * t297 * t46;
    (t10044, t10047, t10063, t10121, t10213, t10257)
}
