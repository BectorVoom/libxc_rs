//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1274/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1274<F: Float>(t1975: F, t9999: F, t3068: F, t3087: F, t10052: F, t10060: F, t1967: F, t27443: F, t3096: F, t3876: F, t6088: F, t6116: F, t623: F, t627: F, t74: F, t79: F, t8061: F, t81: F, t8102: F, t8103: F, t8109: F, t8122: F, t8125: F, t82: F) -> (F, F, F) {
    let t27534 = t1975 * t9999;
    let t27539 = t3068 * t3068;
    let t27564 = t3087 * t3068;
    let t27571 = -F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t6116 * t3876 * t8103 + t27534 * t3087 / F::cast_from(2.0_f64) + t10052 * t6088 / F::cast_from(4.0_f64) - F::cast_from(8.0_f64) * t27539 * t82 + t1975 * t27539 * t81 / F::cast_from(2.0_f64) - F::cast_from(2.0_f64) * t79 * t27539 * t81 + t623 * t27443 * t81 / F::cast_from(2.0_f64) - F::cast_from(8.0_f64) * t10060 * t8061 - F::cast_from(4.0_f64) * t8125 * t3876 - F::cast_from(8.0_f64) * t3096 * t9999 - F::cast_from(4.0_f64) * t627 * t27443 - t74 * t27443 * t81 - F::cast_from(4.0_f64) * t1967 * t27539 * t81 + t8122 * t27564 / F::cast_from(2.0_f64) + F::cast_from(30.0_f64) * t8102 * t27564 - F::cast_from(10.0_f64) * t8109 * t27564;
    (t27539, t27564, t27571)
}
