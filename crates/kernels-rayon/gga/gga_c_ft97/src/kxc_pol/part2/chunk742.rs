//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 742/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk742(t11021: f64, t11023: f64, t11025: f64, t11019: f64, t11036: f64, t7775: f64, t7778: f64, t7782: f64, t7820: f64, t8192: f64, t8195: f64, t11043: f64) -> (f64, f64) {
    let t11646 = 2.0_f64 / 27.0_f64 * t11021;
    let t11647 = 4.0_f64 / 27.0_f64 * t11023;
    let t11648 = 4.0_f64 / 81.0_f64 * t11025;
    let t11656 = t11019 / 9.0_f64 - t11646 - t11647 + t11648 - 8.0_f64 / 81.0_f64 * t7775 + t7778 / 27.0_f64 + 2.0_f64 / 81.0_f64 * t7782 - 2.0_f64 / 27.0_f64 * t7820 - 8.0_f64 / 27.0_f64 * t8192 + t8195 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t11036;
    let t11659 = 4.0_f64 / 81.0_f64 * t11043;
    (t11656, t11659)
}
