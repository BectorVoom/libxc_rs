//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1403/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1403<F: Float>(t127759: F, t127763: F, t127767: F, t127770: F, t127773: F, t127776: F, t127779: F, t127781: F, t127784: F, t127789: F, t127791: F, t127796: F, t127800: F, t127803: F, t127806: F, t127808: F, t127812: F, t127816: F, t127820: F, t127824: F, t127828: F, t127831: F, t99607: F) -> (F, F) {
    let t128293 = 2.0 / 9.0 * t127759 + 8.0 * t127763 + t127767 / 9.0 - t127770 / 9.0 - 2.0 / 9.0 * t127773 - 2.0 / 9.0 * t127776 + t127779 / 3.0 + 2.0 / 3.0 * t127781 - 4.0 / 9.0 * t127784 - t127789 / 8.0 + 4.0 / 27.0 * t127791;
    let t128305 = -2.0 * t127796 + 4.0 / 3.0 * t127800 + 8.0 / 27.0 * t99607 + 2.0 / 9.0 * t127803 + t127806 / 3.0 + 4.0 / 9.0 * t127808 - t127812 / 18.0 - t127816 / 18.0 + t127820 / 9.0 + t127824 + t127828 / 24.0 - 2.0 / 27.0 * t127831;
    (t128293, t128305)
}
