//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 685/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk685<F: Float>(t3199: F, t405: F, t921: F, t758: F, t1220: F, t2346: F, t2350: F, t2380: F, t3165: F, t3172: F, t3174: F, t3177: F, t3181: F, t3185: F, t3189: F, t3193: F, t3196: F, t385: F, t909: F, t918: F) -> (F, F) {
    let t3201 = t405 * t3199 * t921;
    let t3202 = t758 * t3201;
    let t3205 = -t3165 / F::cast_from(108.0_f64) + t1220 * t909 / F::cast_from(36.0_f64) - t2346 - t2350 / F::cast_from(288.0_f64) - t3172 / F::cast_from(288.0_f64) + t3174 * t3177 / F::cast_from(48.0_f64) - t385 * t3181 / F::cast_from(96.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t3185 * t3189 + F::cast_from(0.14291339372689912324e-3_f64) * t3193 - F::cast_from(0.42874018118069736972e-3_f64) * t2380 * t3196 + F::cast_from(0.21437009059034868486e-3_f64) * t918 * t3202;
    (t3201, t3205)
}
