//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1232/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1232<F: Float>(t30793: F, t35004: F, t35012: F, t35018: F, t35022: F, t35024: F, t37321: F, t39581: F, t39585: F, t39587: F, t39592: F, t39594: F, t39599: F, t39601: F, t39605: F, t39607: F, t39609: F) -> F {
    let t41721 = F::cast_from(0.21437009059034868486e-2_f64) * t39581 + F::cast_from(0.14291339372689912324e-2_f64) * t39585 - F::cast_from(0.68598428988911579156e-2_f64) * t39587 - F::cast_from(0.12579236915841660828e-2_f64) * t39592 + t37321 + F::cast_from(0.68598428988911579156e-2_f64) * t39594 + F::cast_from(0.42874018118069736972e-3_f64) * t39599 + t39601 / F::cast_from(8.0_f64) - F::cast_from(0.37737710747524982482e-2_f64) * t30793 - t35004 + F::cast_from(0.68598428988911579156e-2_f64) * t39605 + F::cast_from(0.68598428988911579156e-2_f64) * t39607 - F::cast_from(0.37737710747524982482e-1_f64) * t39609 - t35012 + t35018 + F::cast_from(0.11433071498151929859e-2_f64) * t35022 - t35024;
    t41721
}
