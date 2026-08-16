//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 908/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk908(t1796: f64, t3008: f64, t3009: f64, t1808: f64, t3014: f64, t1802: f64, t3: f64, t545: f64, t3015: f64, t39: f64, t574: f64, t577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7962 = t3008 * t3009 * t1796;
    let t7966 = t3014 * t3009 * t1808;
    let t7969 = t1802 * t3;
    let t7971 = t3014 * t7969 * t545;
    let t7975 = t3014 * t3015 * t1796;
    let t7978 = t574 * t39;
    let t7979 = t7978 * t577;
    (t7962, t7966, t7971, t7975, t7978, t7979)
}
