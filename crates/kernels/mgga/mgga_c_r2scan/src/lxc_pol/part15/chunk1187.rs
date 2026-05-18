//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1187/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1187<F: Float>(t38166: F, t10708: F, t7262: F, t3281: F, t7470: F, t10848: F, t11760: F, t2207: F, t38170: F, t38176: F, t38177: F, t38183: F, t38185: F, t38190: F, t38191: F, t38193: F) -> F {
    let t40248 = F::new(0.84755945902752848174e0) * t38166;
    let t40251 = t10708 * t7262;
    let t40257 = t3281 * t7470;
    let t40258 = F::new(0.10975748638225852664e-1) * t40257;
    let t40260 = t2207 * t11760 * t10848;
    let t40261 = F::new(0.13972381860938637374e0) * t40260;
    let t40262 = t40248 + F::new(0.45022119329691164872e0) * t38170 + t38176 - F::new(0.65854491829355115987e-1) * t38177 - F::new(0.17853738449127731614e0) * t40251 - F::new(0.32927245914677557994e-1) * t38183 + F::new(0.29272321618148349056e-1) * t38185 + t38190 - F::new(0.54878743191129263322e-2) * t38191 + F::new(0.54878743191129263322e-2) * t38193 + t40258 - t40261;
    t40262
}
