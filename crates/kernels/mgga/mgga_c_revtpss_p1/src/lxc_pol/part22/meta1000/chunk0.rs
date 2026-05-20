//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3398/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3398<F: Float>(t52126: F, t52128: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63519: F, t63522: F, t63525: F, t63528: F, t63531: F, t63533: F, t63536: F, t63538: F) -> F {
    let t63797 = -F::cast_from(0.36793333333333333334e0_f64) * t52126 + F::cast_from(0.49057777777777777779e0_f64) * t52128 + F::cast_from(0.20128333333333333334e0_f64) * t63447 - F::new(0.301925e0) * t63451 - F::cast_from(0.89459259259259259257e-1_f64) * t63453 - F::cast_from(0.40256666666666666666e0_f64) * t63457 + F::cast_from(0.26837777777777777777e0_f64) * t63459 + F::new(0.16557e0) * t63519 + F::new(0.16557e0) * t63522 - F::new(0.27595e-1) * t63525 - F::cast_from(0.36793333333333333333e-1_f64) * t63528 - F::new(0.82785e-1) * t63531 - F::cast_from(0.30661111111111111112e-1_f64) * t63533 - F::new(0.5519e-1) * t63536 + F::cast_from(0.18396666666666666667e0_f64) * t63538;
    t63797
}
