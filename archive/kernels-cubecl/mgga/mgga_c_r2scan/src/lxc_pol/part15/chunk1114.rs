//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1114/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1114<F: Float>(t11802: F, t37685: F, t39352: F, t39355: F, t39358: F, t39362: F, t39364: F, t39367: F, t39370: F, t39373: F, t39376: F, t39379: F) -> F {
    let t39381 = t37685 * t11802;
    let t39383 = -F::cast_from(0.16463622957338778997e0_f64) * t39352 - F::cast_from(0.14282990759302185291e-1_f64) * t39355 - F::cast_from(0.57131963037208741166e-1_f64) * t39358 - t39362 + F::cast_from(0.43341108700271342816e-1_f64) * t39364 + F::cast_from(0.13002332610081402845e0_f64) * t39367 - F::cast_from(0.86682217400542685632e-1_f64) * t39370 - F::cast_from(0.86682217400542685632e-1_f64) * t39373 + F::cast_from(0.86682217400542685632e-1_f64) * t39376 + F::cast_from(0.2600466522016280569e0_f64) * t39379 + F::cast_from(0.86682217400542685632e-1_f64) * t39381;
    t39383
}
