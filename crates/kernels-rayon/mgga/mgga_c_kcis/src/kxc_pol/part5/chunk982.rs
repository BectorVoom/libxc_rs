//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 982/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk982(t11633: f64, t531: f64, t10338: f64, t1474: f64, t1444: f64, t461: f64, t543: f64, t1479: f64, t3251: f64, t1484: f64, t11402: f64, t513: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11634 = t11633 * t531;
    let t11640 = t10338 * t1474;
    let t11670 = 1.0_f64 / t461 / t1444;
    let t11671 = t11670 * t543;
    let t11721 = t3251 * t1479;
    let t11723 = t3251 * t1484;
    let t11727 = t11402 * t513;
    (t11634, t11640, t11670, t11671, t11721, t11723, t11727)
}
