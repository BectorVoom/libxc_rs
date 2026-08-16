//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2895/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2895<F: Float>(t2793: F, t2842: F, t5727: F, t4395: F, t2792: F, t913: F, t10650: F, t14332: F, t14436: F, t14450: F, t1581: F, t2886: F, t2888: F, t4472: F, t48776: F, t48783: F, t48854: F, t49404: F, t49478: F, t60354: F, t60359: F, t60360: F, t60371: F, t60374: F, t60377: F, t60381: F, t60384: F, t60387: F, t60391: F, t931: F) -> (F, F, F, F, F) {
    let t60394 = F::cast_from(6.0_f64) * t2842 * t5727 * t2793;
    let t60395 = t4395 * t4395;
    let t60398 = F::cast_from(4.0_f64) * t2792 * t60395 * t913;
    let t60400 = F::cast_from(1.0_f64) * t10650 * t5727;
    let t60401 = F::cast_from(0.11696447245269292414e1_f64) * t49404 * t1581 + F::cast_from(0.23392894490538584828e1_f64) * t14332 * t4472 - t60354 - F::cast_from(0.77193501593724168323e3_f64) * t48776 * t14436 + t60359 + F::cast_from(0.64327917994770140268e2_f64) * t2886 * t60360 * t2888 + F::cast_from(0.14035736694323150897e2_f64) * t48783 * t14450 + F::cast_from(0.8276162067083744048e4_f64) * t49478 * t48854 * t931 + t60371 + t60374 - t60377 - t60381 - t60384 - t60387 - t60391 - t60394 + t60398 - t60400;
    (t60394, t60395, t60398, t60400, t60401)
}
