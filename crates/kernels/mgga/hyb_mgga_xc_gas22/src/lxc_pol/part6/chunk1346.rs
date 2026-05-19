//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1346/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1346<F: Float>(t2234: F, t2236: F, t29076: F, t2187: F, t4108: F, t2190: F, t2189: F, t4114: F, t6579: F, t8731: F, t8906: F, t10645: F, t20846: F) -> (F, F, F, F, F) {
    let t29384 = F::cast_from(0.32163958997385070134e2_f64) * t2234 * t29076 * t2236;
    let t29385 = t4108 * t2187;
    let t29387 = F::new(2.0) * t29385 * t2190;
    let t29392 = F::new(24.0) * t6579 * t4114 * t2189;
    let t29394 = F::new(12.0) * t8906 * t8731;
    let t29396 = F::cast_from(0.1929837539843104208e3_f64) * t20846 * t10645;
    (t29384, t29387, t29392, t29394, t29396)
}
