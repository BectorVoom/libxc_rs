//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1460/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1460<F: Float>(t22393: F, t22418: F, t22430: F, t22459: F, t1343: F, t1353: F, t13599: F, t13600: F, t1450: F, t1868: F, t198: F, t21901: F, t21905: F, t21933: F, t21937: F, t21969: F, t4139: F, t532: F, t5532: F, t5536: F, t5591: F, t5627: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> F {
    let t22461 = t22393 + t22418 + t22430 + t22459;
    let t22465 = t1450 * t198 * t22461 * t532 + F::cast_from(3.0_f64) * t1343 * t198 * t21969 + F::cast_from(3.0_f64) * t1353 * t21937 * t4139 + F::cast_from(6.0_f64) * t13600 * t1868 * t4139 + F::cast_from(6.0_f64) * t4139 * t5532 * t5591 + F::cast_from(12.0_f64) * t5532 * t5536 * t5627 - t13599 + t21901 - t21905 + t21933 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t9389 - t9391;
    t22465
}
