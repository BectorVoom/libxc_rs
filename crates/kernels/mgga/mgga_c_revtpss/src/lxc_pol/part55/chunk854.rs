//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 854/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk854<F: Float>(t14587: F, t28425: F, t26497: F, t4481: F, t26550: F, t27349: F, t14495: F, t27312: F, t212: F, t7997: F, t780: F, t689: F, t2067: F, t25391: F, t26541: F, t26545: F, t26557: F, t26558: F, t26561: F, t26564: F, t26578: F, t27199: F, t27275: F, t27353: F, t7415: F) -> (F,) {
    let t28426 = t28425 * t14587;
    let t28434 = t26497 * t4481;
    let t28436 = t26550 * t27349;
    let t28439 = t26550 * t14495;
    let t28442 = t26550 * t27312;
    let t28447 = t212 * t7997;
    let t28448 = t28447 * t780;
    let t28449 = t689 * t28448;
    let t28453 = -0.8673628188205199462e0 * t27353 * t28426 - 0.14456046980341999104e-1 * t26541 + 0.72280234901709995518e-2 * t26545 - t26557 - 0.4336814094102599731e0 * t27275 * t2067 - 0.12851425765524037203e-1 * t26558 - 0.9757440539382783019e-2 * t28434 - 0.8673628188205199462e0 * t25391 * t28436 + 0.4336814094102599731e0 * t27353 * t28439 - 0.8673628188205199462e0 * t25391 * t28442 + 0.8673628188205199462e0 * t27199 * t7415 - 0.54878743191129263322e-2 * t28449 + 0.54878743191129263322e-2 * t26561 + 0.9757440539382783019e-2 * t26564 + t26578;
    (t28453,)
}
