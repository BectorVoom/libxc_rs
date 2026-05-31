//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1289/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1289<F: Float>(t1196: F, t20397: F, t300: F, t6513: F, t1198: F, t16784: F, t1765: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F) -> (F, F, F, F) {
    let t20399 = F::cast_from(0.34631718211362927518e2_f64) * t1196 * t20397;
    let t20400 = t300 * t6513;
    let t20402 = F::cast_from(0.5848223622634646207e0_f64) * t20400 * t1198;
    let t20404 = F::cast_from(0.11696447245269292414e1_f64) * t16784 * t1765;
    let t20425 = F::cast_from(0.66437037037037037037e-1_f64) * t20283 - F::cast_from(0.19931111111111111111e0_f64) * t20285 - F::cast_from(0.99655555555555555557e-1_f64) * t20287 + F::cast_from(0.29896666666666666667e0_f64) * t20290 + F::cast_from(0.33218518518518518518e0_f64) * t20295 - F::cast_from(0.11958666666666666667e1_f64) * t20300 - F::cast_from(0.39862222222222222222e0_f64) * t20304 + F::cast_from(0.17938e1_f64) * t20308 + F::cast_from(0.11958666666666666667e1_f64) * t20312 - F::cast_from(0.19931111111111111111e0_f64) * t20315 + F::cast_from(0.59793333333333333334e0_f64) * t20320;
    (t20399, t20402, t20404, t20425)
}
