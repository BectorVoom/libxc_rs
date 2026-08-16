//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1055/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1055(t3881: f64, t6092: f64, t3876: f64, t81: f64, t6116: f64, t1967: f64, t1975: f64, t6127: f64, t1211: f64, t82: f64, t79: f64, t3068: f64, t3086: f64, t3087: f64, t3093: f64, t3096: f64, t3099: f64, t3882: f64, t623: f64, t627: f64, t74: f64, t8080: f64, t9999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10013 = t6092 * t3881;
    let t10022 = t81 * t3876;
    let t10043 = t6116 * t3881;
    let t10046 = t1967 * t3876;
    let t10052 = t1975 * t3876;
    let t10057 = t6127 * t3881;
    let t10060 = t1211 * t82;
    let t10063 = t79 * t1211;
    let t10073 = 15.0_f64 / 2.0_f64 * t3882 * t3087 - 4.0_f64 * t3086 * t8080 - 5.0_f64 / 2.0_f64 * t10043 * t3087 - 2.0_f64 * t10046 * t3087 + t623 * t9999 * t81 / 2.0_f64 + t10052 * t3087 / 4.0_f64 + t3093 * t8080 / 2.0_f64 + t10057 * t3087 / 8.0_f64 - 8.0_f64 * t10060 * t3068 - 2.0_f64 * t10063 * t8080 - 4.0_f64 * t3096 * t3876 - t3099 * t10022 - 4.0_f64 * t627 * t9999 - t74 * t9999 * t81;
    (t10013, t10022, t10043, t10046, t10052, t10057, t10060, t10063, t10073)
}
