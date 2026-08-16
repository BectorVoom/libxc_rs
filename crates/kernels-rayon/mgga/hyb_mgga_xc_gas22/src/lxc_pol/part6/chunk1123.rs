//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1123/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1123(t11056: f64, t11066: f64, t987: f64, t10878: f64, t10880: f64, t10882: f64, t10884: f64, t10886: f64, t10956: f64, t10958: f64, t1434: f64, t2533: f64, t4284: f64, t4297: f64, t4300: f64, t7140: f64, t7159: f64, t9199: f64, t979: f64) -> (f64, f64, f64) {
    let t11067 = t11056 + t11066;
    let t11068 = t11067 * t987;
    let t11075 = -2.0_f64 * t7140 * t4284 + 1.0_f64 * t2533 * t4297 + 1.0_f64 * t979 * t11068 + 0.32163958997385070134e2_f64 * t7159 * t4300 - t10878 - t10880 - t10882 + t10884 - t10886 - t10956 - t10958 + 0.11696447245269292414e1_f64 * t9199 * t1434;
    (t11067, t11068, t11075)
}
