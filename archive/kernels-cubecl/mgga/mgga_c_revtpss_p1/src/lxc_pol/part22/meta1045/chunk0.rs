//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3659/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3659<F: Float>(t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F) -> F {
    let t69153 = F::cast_from(0.68493333333333333332e-1_f64) * t68297 + F::cast_from(0.34246666666666666666e-1_f64) * t68301 + F::cast_from(0.10274e0_f64) * t68305 - F::cast_from(0.50735802469135802467e-1_f64) * t68310 + F::cast_from(0.76103703703703703701e-2_f64) * t68332 + F::cast_from(0.1522074074074074074e-1_f64) * t68334 + F::cast_from(0.4566222222222222222e-1_f64) * t68336 + F::cast_from(0.19025925925925925925e-1_f64) * t68342 + F::cast_from(0.2283111111111111111e0_f64) * t68347 - F::cast_from(0.68493333333333333331e-1_f64) * t68350 - F::cast_from(0.41095999999999999999e0_f64) * t68353 - F::cast_from(0.2283111111111111111e-1_f64) * t68357 + F::cast_from(0.41096e0_f64) * t68360;
    t69153
}
