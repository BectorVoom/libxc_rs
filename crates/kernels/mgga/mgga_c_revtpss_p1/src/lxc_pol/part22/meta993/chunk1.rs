//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3380/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3380<F: Float>(t2439: F, t6135: F, t52126: F, t52128: F, t63447: F, t63451: F, t63453: F, t63457: F, t63459: F, t63519: F, t63522: F, t63525: F, t63528: F, t63531: F, t63533: F, t63536: F) -> (F, F) {
    let t63538 = t2439 * t6135;
    let t63540 = -F::cast_from(0.36514074074074074074e0_f64) * t52126 + F::cast_from(0.48685432098765432099e0_f64) * t52128 + F::cast_from(0.19931111111111111111e0_f64) * t63447 - F::cast_from(0.29896666666666666667e0_f64) * t63451 - F::cast_from(0.88582716049382716049e-1_f64) * t63453 - F::cast_from(0.39862222222222222222e0_f64) * t63457 + F::cast_from(0.26574814814814814815e0_f64) * t63459 + F::cast_from(0.16431333333333333333e0_f64) * t63519 + F::cast_from(0.16431333333333333333e0_f64) * t63522 - F::cast_from(0.27385555555555555556e-1_f64) * t63525 - F::cast_from(0.36514074074074074075e-1_f64) * t63528 - F::cast_from(0.82156666666666666667e-1_f64) * t63531 - F::cast_from(0.30428395061728395062e-1_f64) * t63533 - F::cast_from(0.54771111111111111112e-1_f64) * t63536 + F::cast_from(0.18257037037037037037e0_f64) * t63538;
    (t63538, t63540)
}
