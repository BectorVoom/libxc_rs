//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 930/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk930<F: Float>(t1270: F, t1282: F, t172: F, t184: F, t2104: F, t2112: F, t2116: F, t2133: F, t2144: F, t3227: F, t3231: F, t3232: F, t3264: F, t6363: F, t740: F, t742: F, t756: F, t8354: F, t8367: F, t8370: F, t8373: F, t8431: F) -> F {
    let t8434 = F::cast_from(7.0_f64) / F::cast_from(2.0_f64) * t2133 * t3232 - t8367 * t3232 / F::cast_from(2.0_f64) - t8370 * t3232 / F::cast_from(4.0_f64) - t3231 * t8373 - F::cast_from(6.0_f64) * t6363 * t1270 * t2112 + F::cast_from(4.0_f64) * t2116 * t3227 * t740 + F::cast_from(2.0_f64) * t2116 * t1270 * t2104 - t742 * t8354 + F::cast_from(2.0_f64) * t8354 * t184 + F::cast_from(4.0_f64) * t3227 * t756 + F::cast_from(2.0_f64) * t1270 * t2144 + F::cast_from(2.0_f64) * t2104 * t1282 + F::cast_from(4.0_f64) * t740 * t3264 + F::cast_from(2.0_f64) * t172 * t8431;
    t8434
}
