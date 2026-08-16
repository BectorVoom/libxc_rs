//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 905/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk905(t1230: f64, t668: f64, t545: f64, t1796: f64, t2997: f64, t1189: f64, t6012: f64, t1890: f64, t3011: f64, t3017: f64, t25: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7920 = t1230 * t668;
    let t7921 = t7920 * t545;
    let t7925 = t2997 * t1796;
    let t7933 = t6012 * t1189;
    let t7936 = 2.0_f64 / 243.0_f64 * t1890 * t3011;
    let t7938 = 2.0_f64 / 81.0_f64 * t1890 * t3017;
    let t7940 = 1.0_f64 / t25 / t460;
    (t7920, t7921, t7925, t7933, t7936, t7938, t7940)
}
