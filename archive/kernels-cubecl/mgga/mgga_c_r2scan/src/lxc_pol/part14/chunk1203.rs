//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1203/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1203<F: Float>(t39358: F, t39361: F, t39352: F, t39364: F, t39367: F, t39370: F, t39373: F, t39376: F, t39379: F, t39381: F, t41352: F, t39395: F) -> (F, F) {
    let t41353 = F::cast_from(0.11426392607441748234e0_f64) * t39358;
    let t41354 = F::cast_from(0.46230515946956099004e0_f64) * t39361;
    let t41362 = -F::cast_from(0.32927245914677557992e0_f64) * t39352 - t41352 - t41353 - t41354 + F::cast_from(0.86682217400542685632e-1_f64) * t39364 + F::cast_from(0.2600466522016280569e0_f64) * t39367 - F::cast_from(0.17336443480108537126e0_f64) * t39370 - F::cast_from(0.17336443480108537126e0_f64) * t39373 + F::cast_from(0.17336443480108537126e0_f64) * t39376 + F::cast_from(0.5200933044032561138e0_f64) * t39379 + F::cast_from(0.17336443480108537126e0_f64) * t39381;
    let t41367 = F::cast_from(0.25610080155860322884e0_f64) * t39395;
    (t41362, t41367)
}
