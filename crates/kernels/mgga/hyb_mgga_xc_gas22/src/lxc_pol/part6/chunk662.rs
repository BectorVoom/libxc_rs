//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 662/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk662<F: Float>(t1270: F, t1282: F, t172: F, t184: F, t2116: F, t3227: F, t3231: F, t3232: F, t3235: F, t3264: F, t740: F, t742: F, t756: F) -> F {
    let t3267 = -t3231 * t3232 / F::cast_from(2.0_f64) + F::cast_from(2.0_f64) * t2116 * t3235 - t742 * t3227 + F::cast_from(2.0_f64) * t3227 * t184 + F::cast_from(2.0_f64) * t1270 * t756 + F::cast_from(2.0_f64) * t740 * t1282 + F::cast_from(2.0_f64) * t172 * t3264;
    t3267
}
