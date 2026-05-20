//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2173/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2173<F: Float>(t689: F, t6896: F, t7242: F, t22399: F, t26054: F, t108282: F, t25930: F, t27837: F, t27841: F, t27972: F, t543: F, t6843: F, t7274: F, t7295: F, t7298: F, t7301: F, t7921: F, t94784: F, t94807: F, t94820: F, t94842: F, t97875: F, t98010: F, t98011: F, t98029: F, t98050: F) -> F {
    let t108411 = t689 * t7242 * t6896;
    let t108422 = t26054 * t22399;
    let t108425 = F::cast_from(0.17347256376410398924e1_f64) * t98050 * t7921 + t94784 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t97875 * t27972 - t98010 + F::cast_from(0.34270468708064099208e-1_f64) * t98011 - F::cast_from(0.52041769129231196772e1_f64) * t27837 * t27841 + F::cast_from(0.17135234354032049604e-2_f64) * t94807 - F::cast_from(0.10975748638225852664e-1_f64) * t108411 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t7274 * t6843 * t543 - F::cast_from(0.24093411633903331839e-3_f64) * t94820 + F::cast_from(0.38549458614245330944e-1_f64) * t98029 + F::cast_from(0.8673628188205199462e0_f64) * t108282 * t7298 - F::cast_from(0.9757440539382783019e-2_f64) * t108422 + F::cast_from(0.96373646535613327357e-2_f64) * t94842;
    t108425
}
