//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1274/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1274(t1975: f64, t9999: f64, t3068: f64, t3087: f64, t10052: f64, t10060: f64, t1967: f64, t27443: f64, t3096: f64, t3876: f64, t6088: f64, t6116: f64, t623: f64, t627: f64, t74: f64, t79: f64, t8061: f64, t81: f64, t8102: f64, t8103: f64, t8109: f64, t8122: f64, t8125: f64, t82: f64) -> (f64, f64, f64) {
    let t27534 = t1975 * t9999;
    let t27539 = t3068 * t3068;
    let t27564 = t3087 * t3068;
    let t27571 = -5.0_f64 / 2.0_f64 * t6116 * t3876 * t8103 + t27534 * t3087 / 2.0_f64 + t10052 * t6088 / 4.0_f64 - 8.0_f64 * t27539 * t82 + t1975 * t27539 * t81 / 2.0_f64 - 2.0_f64 * t79 * t27539 * t81 + t623 * t27443 * t81 / 2.0_f64 - 8.0_f64 * t10060 * t8061 - 4.0_f64 * t8125 * t3876 - 8.0_f64 * t3096 * t9999 - 4.0_f64 * t627 * t27443 - t74 * t27443 * t81 - 4.0_f64 * t1967 * t27539 * t81 + t8122 * t27564 / 2.0_f64 + 30.0_f64 * t8102 * t27564 - 10.0_f64 * t8109 * t27564;
    (t27539, t27564, t27571)
}
