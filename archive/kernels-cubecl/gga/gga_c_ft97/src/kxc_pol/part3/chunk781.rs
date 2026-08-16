//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 781/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk781<F: Float>(t11811: F, t3214: F, t11810: F, t16083: F, t16086: F, t16090: F, t16095: F, t16099: F, t16103: F, t16107: F, t16112: F, t16117: F, t16122: F, t16126: F, t16129: F, t16133: F, t16137: F, t1901: F, t446: F) -> F {
    let t16140 = t11811 * t3214;
    let t16141 = t11810 * t16140;
    let t16144 = -F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t16083 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16086 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16090 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16095 - t446 * t16099 / F::cast_from(9.0_f64) - t446 * t16103 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t446 * t16107 - F::cast_from(2.0_f64) * t446 * t16112 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16117 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t16122 - t16126 / F::cast_from(9.0_f64) - t446 * t16129 / F::cast_from(3.0_f64) - t446 * t16133 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t16137 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t16141;
    t16144
}
