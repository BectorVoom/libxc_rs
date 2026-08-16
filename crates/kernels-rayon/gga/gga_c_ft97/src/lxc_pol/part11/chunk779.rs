//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 779/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk779(t1882: f64, t2889: f64, t2857: f64, t319: f64, t9578: f64, t10394: f64, t10400: f64, t10276: f64, t848: f64, t9938: f64, t2787: f64, t458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10545 = t1882 * t2889;
    let t10548 = t2857 * t319 * t9578;
    let t10552 = t10394 / 3.0_f64;
    let t10553 = 4.0_f64 / 9.0_f64 * t10400;
    let t10555 = 2.0_f64 / 3.0_f64 * t10276;
    let t10556 = t848 * t9938;
    let t10559 = t458 * t2787;
    (t10545, t10548, t10552, t10553, t10555, t10556, t10559)
}
