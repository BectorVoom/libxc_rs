//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1175/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1175<F: Float>(t15262: F, t15348: F, t15403: F, t15516: F, t300: F, t3007: F, t4724: F, t981: F, t3022: F, t4734: F, t3011: F, t4707: F, t4733: F, t15495: F, t15234: F, t964: F, t973: F) -> (F, F, F, F, F, F) {
    let t15519 = t300 * (t15262 + t15348 + t15403 + t15516);
    let t15520 = t4724 * t3007;
    let t15522 = 0.11696447245269292414e1 * t981 * t15520;
    let t15524 = 0.34631718211362927518e2 * t3022 * t4734;
    let t15525 = t3011 * t4707;
    let t15526 = t15525 * t4733;
    let t15528 = 0.34631718211362927518e2 * t981 * t15526;
    let t15530 = 0.19751673498613801407e-1 * t300 * t15495;
    let t15534 = t964 * t15234 * t973;
    (t15519, t15522, t15524, t15528, t15530, t15534)
}
