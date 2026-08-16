//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1346/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1346(t2234: f64, t2236: f64, t29076: f64, t2187: f64, t4108: f64, t2190: f64, t2189: f64, t4114: f64, t6579: f64, t8731: f64, t8906: f64, t10645: f64, t20846: f64) -> (f64, f64, f64, f64, f64) {
    let t29384 = 0.32163958997385070134e2_f64 * t2234 * t29076 * t2236;
    let t29385 = t4108 * t2187;
    let t29387 = 2.0_f64 * t29385 * t2190;
    let t29392 = 24.0_f64 * t6579 * t4114 * t2189;
    let t29394 = 12.0_f64 * t8906 * t8731;
    let t29396 = 0.1929837539843104208e3_f64 * t20846 * t10645;
    (t29384, t29387, t29392, t29394, t29396)
}
