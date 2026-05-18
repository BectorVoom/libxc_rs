//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1086/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1086<F: Float>(t355: F, t7329: F, t988: F, t1015: F, t115: F, t18485: F, t2326: F, t2328: F, t2337: F, t23490: F, t23495: F, t23503: F, t23510: F, t23513: F, t23519: F, t23520: F, t23523: F, t23531: F, t23537: F, t2433: F, t2554: F, t279: F, t363: F, t5: F, t7313: F, t8289: F, rho0: F, sigma0: F) -> F {
    let t23539 = t355 * t988 * t7329;
    let t23541 = -F::new(1520000.0) / F::new(243.0) * t23490 + F::new(136400.0) / F::new(243.0) * t2554 * t7313 * t1015 - F::new(400.0) / F::new(81.0) * t23495 + F::new(51260000.0) / F::new(729.0) * t2326 * t2328 / t18485 * t2337 + F::new(10472.0) / F::new(81.0) * t355 * t23503 * t115 * t5 * t363 - F::new(2464.0) / F::new(81.0) * t23510 + F::new(200.0) / F::new(81.0) * t2433 * t23513 + F::new(400000000.0) / F::new(6561.0) * t23519 * t23520 * t8289 * sigma0 / t279 / t23523 / rho0 * t1015 - F::new(16.0) / F::new(3.0) * t23531 - t23537 - F::new(160.0) / F::new(81.0) * t23539;
    t23541
}
