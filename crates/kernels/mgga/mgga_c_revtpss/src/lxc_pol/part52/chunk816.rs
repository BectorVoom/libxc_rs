//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 816/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk816<F: Float>(t1955: F, t4469: F, t72: F, t7778: F, t686: F, t7064: F, t1558: F, t231: F, t7048: F, t7076: F, t1949: F, t4423: F, t1959: F, t25297: F, t25303: F, t25307: F, t25311: F, t25333: F, t25337: F, t25340: F, t25353: F, t25356: F, t25383: F, t7070: F, t7775: F) -> (F, F, F, F, F, F, F) {
    let t27275 = t1955 * t4469;
    let t27278 = t7778 * t72;
    let t27279 = t27278 * t686;
    let t27280 = t7064 * t27279;
    let t27286 = t7048 * t1558 * t231;
    let t27287 = t7076 * t27286;
    let t27291 = t1949 * t4423 * t231;
    let t27292 = t7076 * t27291;
    let t27297 = 0.72280234901709995518e-2 * t25297 + t25303 - t25307 + 0.72280234901709995518e-2 * t25311 - 0.4336814094102599731e0 * t27275 * t1959 + t25333 - 0.12851425765524037203e-1 * t27280 - t25337 - 0.54878743191129263322e-2 * t25340 + 0.4336814094102599731e0 * t25383 * t7775 + 0.4336814094102599731e0 * t7070 * t27287 + 0.4336814094102599731e0 * t7070 * t27292 + 0.54878743191129263322e-2 * t25353 + 0.9757440539382783019e-2 * t25356;
    (t27275, t27279, t27286, t27287, t27291, t27292, t27297)
}
