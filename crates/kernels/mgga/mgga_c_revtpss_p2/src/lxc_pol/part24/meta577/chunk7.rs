//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1776/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1776<F: Float>(t6502: F, t1169: F, t1188: F, t12429: F, t12486: F, t12553: F, t17032: F, t17097: F, t24431: F, t24436: F, t3452: F, t3477: F, t3479: F, t3496: F, t3521: F, t3523: F, t6487: F, t69488: F, t90319: F, t90329: F, t90332: F, t90336: F, t90339: F, t90341: F, t90343: F, t90352: F, t90357: F, t90670: F) -> F {
    let t90756 = t6502 * t6502;
    let t90775 = F::cast_from(0.6233709278045326953e3_f64) * t12553 * t90357 * t3523 - F::cast_from(12.0_f64) * t69488 * t6487 + F::cast_from(24.0_f64) * t17032 * t24431 - F::cast_from(24.0_f64) * t12429 * t90670 * t1169 - F::cast_from(6.0_f64) * t3452 * t90756 * t1169 + F::cast_from(0.96491876992155210402e2_f64) * t3477 * t90756 * t3479 + F::cast_from(0.14035736694323150897e2_f64) * t17097 * t24436 - F::cast_from(0.14035736694323150897e2_f64) * t12486 * t90357 * t1188 - F::cast_from(0.35089341735807877242e1_f64) * t3496 * t90352 * t1188 + F::cast_from(0.51947577317044391277e2_f64) * t3521 * t90352 * t3523 - t90329 + t90332 + t90336 - t90339 - t90341 - t90343 - F::cast_from(0.19751673498613801407e-1_f64) * t90319;
    t90775
}
