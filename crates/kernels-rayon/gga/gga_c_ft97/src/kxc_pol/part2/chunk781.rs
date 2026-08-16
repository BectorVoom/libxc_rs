//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 781/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk781(t2253: f64, t3655: f64, t12143: f64, t12144: f64, t12148: f64, t12152: f64, t12155: f64, t12158: f64, t12162: f64, t12164: f64, t12165: f64, t12171: f64, t12174: f64, t12177: f64, t12181: f64, t12186: f64, t12190: f64, t12193: f64, t12198: f64, t12201: f64, t12204: f64, t12236: f64, t2265: f64, t3628: f64, t631: f64) -> f64 {
    let t12240 = 2.0_f64 / 3.0_f64 * t2253 * t3655;
    let t12241 = 2.0_f64 / 9.0_f64 * t12143 * t12144 - t2265 * t12148 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t12143 * t12152 - t2265 * t12155 / 3.0_f64 - 4.0_f64 / 3.0_f64 * t12143 * t12158 + t12162 + t12164 + 5.0_f64 / 27.0_f64 * t12165 - 13.0_f64 / 9.0_f64 * t12171 + t12174 - 2.0_f64 / 3.0_f64 * t2265 * t12177 - t2265 * t12181 / 3.0_f64 - t2265 * t12186 / 9.0_f64 - t12190 - 3.0_f64 * t631 * t12193 + 6.0_f64 * t631 * t12198 + t3628 * t12201 / 3.0_f64 + 5.0_f64 / 9.0_f64 * t12204 + t631 * t12236 / 2.0_f64 - t12240;
    t12241
}
