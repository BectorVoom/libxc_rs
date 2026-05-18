//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 957/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk957<F: Float>(t11571: F, t324: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F) -> (F, F) {
    let t11572 = t11571 * t324;
    let t11574 = F::new(0.53272592592592592592e-1) * t11132;
    let t11585 = -t11574 - F::new(0.2283111111111111111e-1) * t11134 + F::new(0.11415555555555555555e-1) * t11136 - F::new(0.34246666666666666665e-1) * t11138 + F::new(0.17123333333333333333e-1) * t11140 - F::new(0.19025925925925925925e-1) * t11147 + F::new(0.68493333333333333331e-1) * t11153 - F::new(0.34246666666666666665e-1) * t11158 - F::new(0.10274e0) * t11162 + F::new(0.10274e0) * t11167 - F::new(0.17123333333333333333e-1) * t11171;
    (t11572, t11585)
}
