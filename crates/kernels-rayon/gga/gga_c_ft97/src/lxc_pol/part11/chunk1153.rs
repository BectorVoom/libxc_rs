//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1153/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1153(t10666: f64, t2801: f64, t10738: f64, t1882: f64, t10741: f64, t192: f64, t33828: f64, t10714: f64, t2399: f64, t2834: f64, t89: f64, t2751: f64, t8232: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44272 = t10666 * t2801;
    let t44276 = t1882 * t10738;
    let t44278 = t1882 * t10741;
    let t44280 = t192 * t33828;
    let t44289 = t1882 * t10714;
    let t44292 = t89 * t2399 * t2834;
    let t44294 = t8232 * t2751;
    (t44272, t44276, t44278, t44280, t44289, t44292, t44294)
}
