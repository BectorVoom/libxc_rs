//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 994/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk994<F: Float>(t30769: F, t30773: F, t30775: F, t30777: F, t34837: F, t34840: F, t34849: F, t34851: F, t34853: F, t34856: F, t37271: F, t39525: F, t39527: F, t39534: F, t39537: F, t39540: F, t39545: F, t39547: F) -> (F,) {
    let t39549 = -t39525 / 16.0 - t34837 + t34840 - 7.0 / 288.0 * t39527 + 0.34299214494455789578e-2 * t30769 + t37271 + 0.21437009059034868486e-3 * t30773 - 0.85748036236139473944e-3 * t30775 + 0.85748036236139473944e-3 * t30777 - 0.11321313224257494744e-1 * t34849 - 0.21437009059034868486e-3 * t39534 - 0.21437009059034868486e-3 * t39537 - 0.21437009059034868486e-3 * t39540 + 0.80031500487063509016e-2 * t34851 - 0.80031500487063509016e-2 * t34853 + t34856 - 0.14291339372689912324e-3 * t39545 - 0.10718504529517434243e-3 * t39547;
    (t39549,)
}
