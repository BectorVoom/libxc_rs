//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1193/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1193<F: Float>(t101812: F, t101824: F, t116735: F, t116739: F, t116743: F, t116747: F, t116752: F, t116756: F, t116760: F, t116764: F, t116767: F, t116771: F, t101876: F, t101879: F, t101883: F, t101899: F, t116776: F, t116780: F, t116782: F, t116786: F, t116790: F, t116793: F, t116796: F, t116799: F) -> (F, F) {
    let t117211 = 8.0 * t116735 - 4.0 * t116739 + 4.0 / 9.0 * t116743 - t116747 / 6.0 + 2.0 / 3.0 * t116752 - 2.0 * t116756 - t116760 / 3.0 - t116764 / 3.0 + t116767 / 12.0 + t101812 - t101824 + 2.0 / 9.0 * t116771;
    let t117222 = t116776 / 18.0 + 16.0 / 27.0 * t101876 + t116780 / 18.0 - 2.0 / 81.0 * t116782 - 2.0 * t116786 + 2.0 / 9.0 * t116790 + t116793 / 24.0 - t116796 / 36.0 - 2.0 / 9.0 * t116799 + 8.0 / 27.0 * t101879 - t101883 - t101899;
    (t117211, t117222)
}
