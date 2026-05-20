//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2197/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2197<F: Float>(t1398: F, t14224: F, t25930: F, t30055: F, t543: F, t7295: F, t7301: F, t94677: F, t94682: F, t97869: F, t97882: F, t97894: F, t97900: F, t97908: F, t97915: F, t97917: F, t97920: F, t97923: F, t97926: F, t98340: F) -> F {
    let t108327 = F::cast_from(0.17135234354032049604e-1_f64) * t94677 + t97869 - F::cast_from(0.23131639038696784278e-2_f64) * t97882 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t98340 * t14224 - F::cast_from(0.13009920719177044025e-2_f64) * t97894 + F::cast_from(0.19274729307122665472e-1_f64) * t97900 + t94682 - t97908 + t97915 + F::cast_from(0.3427046870806409921e-2_f64) * t97917 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t30055 * t1398 * t543 - t97920 + F::cast_from(0.3427046870806409921e-2_f64) * t97923 - F::cast_from(0.19274729307122665472e-1_f64) * t97926;
    t108327
}
