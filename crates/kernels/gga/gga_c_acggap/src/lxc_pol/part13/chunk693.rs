//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 693/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk693<F: Float>(t599: F, t922: F, t142: F, t7450: F, t7388: F, t7391: F, t7394: F, t7397: F, t7398: F, t7403: F, t7406: F, t7407: F, t7409: F, t7411: F, t7416: F, t7420: F, t7424: F, t7429: F, t7435: F, t7438: F, t7442: F, t7445: F, t7449: F) -> (F, F, F) {
    let t7451 = t599 * t922;
    let t7452 = t142 * t7451;
    let t7453 = t7450 * t7452;
    let t7455 = -t7388 - t7391 + t7394 / F::cast_from(192.0_f64) + t7397 + t7398 / F::cast_from(48.0_f64) + t7403 / F::cast_from(32.0_f64) + t7406 - t7407 / F::cast_from(24.0_f64) - t7409 / F::cast_from(48.0_f64) - t7411 / F::cast_from(48.0_f64) - F::cast_from(0.31448092289604152068e-3_f64) * t7416 + F::cast_from(0.15724046144802076034e-3_f64) * t7420 - F::cast_from(0.10718504529517434243e-3_f64) * t7424 - F::cast_from(0.94344276868812456204e-3_f64) * t7429 - t7435 + t7438 / F::cast_from(24.0_f64) - t7442 - F::cast_from(0.22921875e-1_f64) * t7445 - t7449 - F::cast_from(0.4584375e-1_f64) * t7453;
    (t7451, t7452, t7455)
}
