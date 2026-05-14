//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1078/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1078<F: Float>(t30793: F, t35004: F, t35012: F, t35018: F, t35022: F, t35024: F, t37321: F, t39581: F, t39585: F, t39587: F, t39592: F, t39594: F, t39599: F, t39601: F, t39605: F, t39607: F, t39609: F) -> (F,) {
    let t41721 = 0.21437009059034868486e-2 * t39581 + 0.14291339372689912324e-2 * t39585 - 0.68598428988911579156e-2 * t39587 - 0.12579236915841660828e-2 * t39592 + t37321 + 0.68598428988911579156e-2 * t39594 + 0.42874018118069736972e-3 * t39599 + t39601 / 8.0 - 0.37737710747524982482e-2 * t30793 - t35004 + 0.68598428988911579156e-2 * t39605 + 0.68598428988911579156e-2 * t39607 - 0.37737710747524982482e-1 * t39609 - t35012 + t35018 + 0.11433071498151929859e-2 * t35022 - t35024;
    (t41721,)
}
