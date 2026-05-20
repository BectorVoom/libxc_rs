//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2008/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2008<F: Float>(t212: F, t30379: F, t689: F, t780: F, t105936: F, t95537: F, t213: F, t102947: F, t102953: F, t102956: F, t102964: F, t102969: F, t103424: F, t25317: F, t25391: F, t27312: F, t6048: F, t7070: F, t7398: F, t887: F, t95542: F, t95548: F, t95551: F, t95562: F) -> F {
    let t110245 = t689 * t212 * t30379 * t780;
    let t110247 = t95537 * t105936;
    let t110256 = t213 * t30379;
    let t110261 = -F::cast_from(0.54878743191129263322e-2_f64) * t110245 - t102947 - t95542 - F::cast_from(0.51405703062096148813e-1_f64) * t110247 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t7398 * t6048 - t95548 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t103424 * t27312 - t102953 + t102956 - F::cast_from(0.65854491829355115987e0_f64) * t110256 * t887 - F::cast_from(0.96373646535613327357e-2_f64) * t95551 + t102964 - F::cast_from(0.65049603595885220126e-3_f64) * t95562 - t102969;
    t110261
}
