//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 914/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk914(t1967: f64, t3068: f64, t1211: f64, t6116: f64, t1975: f64, t6127: f64, t1947: f64, t82: f64, t79: f64, t3073: f64, t3086: f64, t3087: f64, t3093: f64, t3096: f64, t3099: f64, t6088: f64, t623: f64, t627: f64, t74: f64, t8061: f64, t8080: f64, t81: f64, t8102: f64, t8103: f64) -> (f64, f64, f64, f64, f64) {
    let t8106 = t1967 * t3068;
    let t8109 = t6116 * t1211;
    let t8117 = t1975 * t3068;
    let t8122 = t6127 * t1211;
    let t8125 = t1947 * t82;
    let t8130 = t79 * t1947;
    let t8138 = 15.0_f64 / 2.0_f64 * t8102 * t8103 - 4.0_f64 * t8106 * t3087 - 5.0_f64 / 2.0_f64 * t8109 * t8103 - 2.0_f64 * t3086 * t6088 + t623 * t8061 * t81 / 2.0_f64 + t8117 * t3087 / 2.0_f64 + t3093 * t6088 / 4.0_f64 + t8122 * t8103 / 8.0_f64 - 4.0_f64 * t8125 * t1211 - 8.0_f64 * t3096 * t3068 - t8130 * t3073 - 2.0_f64 * t3099 * t8080 - 4.0_f64 * t627 * t8061 - t74 * t8061 * t81;
    (t8109, t8122, t8125, t8130, t8138)
}
