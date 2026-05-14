//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 691/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk691<F: Float>(t3103: F, t452: F, t986: F, t11811: F, t3214: F, t11810: F, t16083: F, t16086: F, t16090: F, t16095: F, t16099: F, t16103: F, t16107: F, t16112: F, t16117: F, t16122: F, t16126: F, t16129: F, t16133: F, t1901: F, t446: F) -> (F,) {
    let t16137 = t452 * t986 * t3103;
    let t16140 = t11811 * t3214;
    let t16141 = t11810 * t16140;
    let t16144 = -2.0 / 27.0 * t16083 - 2.0 / 3.0 * t446 * t16086 - 2.0 / 3.0 * t446 * t16090 + 2.0 / 3.0 * t446 * t16095 - t446 * t16099 / 9.0 - t446 * t16103 / 9.0 - 2.0 / 27.0 * t446 * t16107 - 2.0 * t446 * t16112 - 2.0 / 3.0 * t446 * t16117 + 4.0 / 3.0 * t446 * t16122 - t16126 / 9.0 - t446 * t16129 / 3.0 - t446 * t16133 / 3.0 - 2.0 / 3.0 * t446 * t16137 - 4.0 / 3.0 * t1901 * t16141;
    (t16144,)
}
