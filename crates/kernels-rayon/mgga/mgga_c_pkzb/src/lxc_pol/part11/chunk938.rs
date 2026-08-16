//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 938/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk938(t2099: f64, t3882: f64, t918: f64, t3898: f64, t6416: f64, t8254: f64, t2371: f64, t3223: f64, t1227: f64, t2411: f64, t300: f64, t3061: f64, t921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10200 = t2099 * t3882;
    let t10201 = t918 * t10200;
    let t10204 = t6416 * t3898;
    let t10205 = t8254 * t10204;
    let t10208 = t2371 * t3223;
    let t10209 = t8254 * t10208;
    let t10212 = t2411 * t1227;
    let t10213 = t300 * t10212;
    let t10214 = t921 * t3061;
    (t10200, t10201, t10204, t10205, t10208, t10209, t10212, t10213, t10214)
}
