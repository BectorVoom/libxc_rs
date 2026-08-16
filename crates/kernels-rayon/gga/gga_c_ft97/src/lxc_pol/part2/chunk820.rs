//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 820/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk820(t12837: f64, t12878: f64, t579: f64, t91: f64, t12306: f64, t12308: f64, t12310: f64, t12285: f64, t12290: f64, t12293: f64, t12296: f64, t12300: f64, t12304: f64, t12315: f64) -> (f64, f64) {
    let t12879 = t12837 + t12878;
    let t12881 = t91 * t579 * t12879;
    let t12889 = 2.0_f64 / 27.0_f64 * t12306;
    let t12890 = 4.0_f64 / 27.0_f64 * t12308;
    let t12891 = 4.0_f64 / 81.0_f64 * t12310;
    let t12893 = t12881 / 6.0_f64 + t12285 / 9.0_f64 + 2.0_f64 / 27.0_f64 * t12290 - 10.0_f64 / 81.0_f64 * t12293 - 8.0_f64 / 27.0_f64 * t12296 + t12300 / 9.0_f64 + 4.0_f64 / 9.0_f64 * t12304 - t12889 - t12890 + t12891 - 2.0_f64 / 9.0_f64 * t12315;
    (t12881, t12893)
}
