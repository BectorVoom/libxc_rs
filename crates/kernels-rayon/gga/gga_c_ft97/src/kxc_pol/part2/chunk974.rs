//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 974/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk974(t10246: f64, t10276: f64, t10279: f64, t10282: f64, t10286: f64, t10394: f64, t10400: f64, t14697: f64, t14701: f64, t14706: f64, t15111: f64, t14895: f64) -> (f64, f64) {
    let t15112 = 4.0_f64 / 3.0_f64 * t14697 + 2.0_f64 / 3.0_f64 * t14701 - 2.0_f64 * t14706 + t10394 / 9.0_f64 - 8.0_f64 / 27.0_f64 * t10400 - 2.0_f64 / 9.0_f64 * t10276 - 2.0_f64 / 27.0_f64 * t10246 - 8.0_f64 / 81.0_f64 * t10279 + t10282 / 27.0_f64 + 2.0_f64 / 81.0_f64 * t10286 - t15111;
    let t15116 = 4.0_f64 / 27.0_f64 * t14895;
    (t15112, t15116)
}
