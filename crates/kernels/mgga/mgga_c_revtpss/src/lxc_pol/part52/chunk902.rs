//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 902/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk902<F: Float>(t27291: F, t7076: F, t1959: F, t25297: F, t25303: F, t25307: F, t25311: F, t25333: F, t25337: F, t25340: F, t25353: F, t25356: F, t25383: F, t27275: F, t27280: F, t27287: F, t7070: F, t7775: F) -> (F, F) {
    let t27292 = t7076 * t27291;
    let t27297 = F::new(0.72280234901709995518e-2) * t25297 + t25303 - t25307 + F::new(0.72280234901709995518e-2) * t25311 - F::new(0.4336814094102599731e0) * t27275 * t1959 + t25333 - F::new(0.12851425765524037203e-1) * t27280 - t25337 - F::new(0.54878743191129263322e-2) * t25340 + F::new(0.4336814094102599731e0) * t25383 * t7775 + F::new(0.4336814094102599731e0) * t7070 * t27287 + F::new(0.4336814094102599731e0) * t7070 * t27292 + F::new(0.54878743191129263322e-2) * t25353 + F::new(0.9757440539382783019e-2) * t25356;
    (t27292, t27297)
}
