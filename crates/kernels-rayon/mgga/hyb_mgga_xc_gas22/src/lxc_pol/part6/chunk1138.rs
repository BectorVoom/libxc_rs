//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1138/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1138(t1159: f64, t4524: f64, t2824: f64, t1123: f64, t4501: f64, t2851: f64, t1129: f64, t1297: f64, t3663: f64, t3662: f64, t4544: f64, t4512: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11266 = t1159 * t4524;
    let t11267 = t11266 * t2824;
    let t11270 = t4501 * t1123;
    let t11271 = t2851 * t11270;
    let t11274 = t4501 * t1129;
    let t11275 = t2851 * t11274;
    let t11278 = t3663 * t1297;
    let t11279 = t3662 * t11278;
    let t11282 = t1159 * t4544;
    let t11283 = t11282 * t2824;
    let t11288 = t4512 * t1123;
    (t11266, t11267, t11270, t11271, t11274, t11275, t11279, t11282, t11283, t11288)
}
