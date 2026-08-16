//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 275/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk275(t1073: f64, t637: f64, t639: f64, t1068: f64, t629: f64, t631: f64, t184: f64, t21: f64, t669: f64, t992: f64, t666: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1075 = t637 * t639 * t1073;
    let t1078 = t629 + t631 * t1068 / 6.0_f64 + t631 * t1075 / 2.0_f64;
    let t1079 = t1078 * t184;
    let t1080 = t1079 * t21;
    let t1087 = t669 * t992;
    let t1089 = t89 * t666 * t1087;
    (t1075, t1078, t1079, t1080, t1087, t1089)
}
