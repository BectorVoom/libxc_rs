//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1055/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1055<F: Float>(t3881: F, t6092: F, t3876: F, t81: F, t6116: F, t1967: F, t1975: F, t6127: F, t1211: F, t82: F, t79: F, t3068: F, t3086: F, t3087: F, t3093: F, t3096: F, t3099: F, t3882: F, t623: F, t627: F, t74: F, t8080: F, t9999: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10013 = t6092 * t3881;
    let t10022 = t81 * t3876;
    let t10043 = t6116 * t3881;
    let t10046 = t1967 * t3876;
    let t10052 = t1975 * t3876;
    let t10057 = t6127 * t3881;
    let t10060 = t1211 * t82;
    let t10063 = t79 * t1211;
    let t10073 = F::cast_from(15.0_f64) / F::cast_from(2.0_f64) * t3882 * t3087 - F::cast_from(4.0_f64) * t3086 * t8080 - F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t10043 * t3087 - F::cast_from(2.0_f64) * t10046 * t3087 + t623 * t9999 * t81 / F::cast_from(2.0_f64) + t10052 * t3087 / F::cast_from(4.0_f64) + t3093 * t8080 / F::cast_from(2.0_f64) + t10057 * t3087 / F::cast_from(8.0_f64) - F::cast_from(8.0_f64) * t10060 * t3068 - F::cast_from(2.0_f64) * t10063 * t8080 - F::cast_from(4.0_f64) * t3096 * t3876 - t3099 * t10022 - F::cast_from(4.0_f64) * t627 * t9999 - t74 * t9999 * t81;
    (t10013, t10022, t10043, t10046, t10052, t10057, t10060, t10063, t10073)
}
