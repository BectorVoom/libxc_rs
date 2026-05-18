//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 862/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk862<F: Float>(t12998: F, t13005: F, t13050: F, t13054: F, t16220: F, t16221: F, t16249: F, t16262: F, t16295: F, t16301: F, t16336: F, t16582: F, t16602: F, t16614: F, t16619: F, t172: F, t188: F, t6318: F, t6321: F, t6324: F, t6328: F, t6330: F, t6332: F, t6457: F, t6465: F, t6526: F, t6638: F, t6644: F, t6696: F, t6741: F, t6747: F, t6750: F, t6753: F, t6771: F, t9431: F, t95: F, t9523: F, t9527: F) -> F {
    let t16623 = param_c1 * (t188 * t16295 / F::new(2.0) + t188 * t16301 / F::new(2.0) + t16336 + t16614 + t16602 - t6747 + t16619 + F::new(35.0) / F::new(3.0) * t9431 + t6753 - t6638 - t6750 - t6332 + F::new(35.0) / F::new(3.0) * t9527 + t16582 - t6318 - t6321 + t6771 + t6465 - F::new(7.0) / F::new(2.0) * t13054 + F::new(3.0) / F::new(2.0) * t13005 - F::new(7.0) * t13050 - F::new(7.0) * t9523 + t6457 + t6526 - t6324 + F::new(0.15506928860942058298e-1) * t95 * t16221 * t172 - t6328 - t6330 + t6741 - t16220 + t16249 + t6696 + t16262 - t6644 - F::new(7.0) / F::new(2.0) * t12998);
    t16623
}
