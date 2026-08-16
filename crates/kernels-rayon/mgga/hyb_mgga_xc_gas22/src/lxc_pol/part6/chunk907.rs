//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 907/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk907(t3025: f64, t7942: f64, t39: f64, t6023: f64, t1179: f64, t6025: f64, t1808: f64, t6033: f64, t3008: f64, t1897: f64, t3: f64, t545: f64) -> (f64, f64, f64, f64, f64) {
    let t7943 = t7942 * t3025;
    let t7945 = t6023 * t39;
    let t7946 = t6025 * t1179;
    let t7948 = t7945 * t7946 * t1808;
    let t7951 = t6033 * t1179;
    let t7953 = t3008 * t7951 * t1808;
    let t7956 = t1897 * t3;
    let t7958 = t3008 * t7956 * t545;
    (t7943, t7945, t7948, t7953, t7958)
}
