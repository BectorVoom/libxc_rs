//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1745/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1745<F: Float>(t28384: F, t7076: F, t1580: F, t7384: F, t689: F, t213: F, t7997: F, t25383: F, t26498: F, t26500: F, t26547: F, t28361: F, t28366: F, t28369: F, t28371: F, t28374: F, t28378: F, t7067: F, t7070: F, t8012: F, t8016: F, t887: F) -> (F, F, F, F, F) {
    let t28385 = t7076 * t28384;
    let t28390 = t7384 * t1580;
    let t28391 = t689 * t28390;
    let t28394 = t213 * t7997;
    let t28397 = F::cast_from(0.72280234901709995518e-2_f64) * t28361 - F::cast_from(0.65854491829355115987e0_f64) * t26547 * t1580 - F::cast_from(0.9757440539382783019e-2_f64) * t26498 - F::cast_from(0.12851425765524037203e-1_f64) * t28366 - F::cast_from(0.72280234901709995518e-2_f64) * t28369 + F::cast_from(0.12851425765524037203e-1_f64) * t28371 + F::cast_from(0.9757440539382783019e-2_f64) * t28374 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t28378 + F::cast_from(0.4336814094102599731e0_f64) * t25383 * t8012 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t28385 - F::cast_from(0.4336814094102599731e0_f64) * t7067 * t8016 + F::cast_from(0.54878743191129263322e-2_f64) * t28391 - F::cast_from(0.72280234901709995518e-2_f64) * t26500 - F::cast_from(0.65854491829355115987e0_f64) * t28394 * t887;
    (t28385, t28390, t28391, t28394, t28397)
}
