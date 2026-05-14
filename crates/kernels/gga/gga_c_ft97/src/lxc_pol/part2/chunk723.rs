//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 723/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk723<F: Float>(t12837: F, t12878: F, t579: F, t91: F, t12306: F, t12308: F, t12310: F, t12285: F, t12290: F, t12293: F, t12296: F, t12300: F, t12304: F, t12315: F, t12327: F, t12319: F, t12322: F, t12325: F, t12332: F, t12336: F, t12340: F, t8796: F, t8805: F, t9065: F, t9068: F) -> (F, F, F) {
    let t12879 = t12837 + t12878;
    let t12881 = t91 * t579 * t12879;
    let t12889 = 2.0 / 27.0 * t12306;
    let t12890 = 4.0 / 27.0 * t12308;
    let t12891 = 4.0 / 81.0 * t12310;
    let t12893 = t12881 / 6.0 + t12285 / 9.0 + 2.0 / 27.0 * t12290 - 10.0 / 81.0 * t12293 - 8.0 / 27.0 * t12296 + t12300 / 9.0 + 4.0 / 9.0 * t12304 - t12889 - t12890 + t12891 - 2.0 / 9.0 * t12315;
    let t12897 = 2.0 / 27.0 * t12327;
    let t12905 = -2.0 / 9.0 * t12319 - 2.0 / 3.0 * t12322 + 4.0 / 9.0 * t12325 - t12897 + 2.0 / 9.0 * t12332 - 4.0 / 9.0 * t12336 - 4.0 / 9.0 * t12340 - 2.0 / 9.0 * t8805 - 8.0 / 27.0 * t9065 + t9068 / 9.0 - 8.0 / 81.0 * t8796;
    (t12881, t12893, t12905)
}
