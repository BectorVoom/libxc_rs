//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 923/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk923(t13891: f64, t13950: f64, t14041: f64, t14091: f64, t14153: f64, t14209: f64, t14251: f64, t14292: f64, t9735: f64, t9701: f64, t13746: f64, t13753: f64) -> (f64, f64, f64, f64, f64) {
    let t14295 = t13891 + t13950 + t14041 + t14091 + t14153 + t14209 + t14251 + t14292;
    let t14317 = 4.0_f64 / 81.0_f64 * t9735;
    let t14318 = 4.0_f64 / 27.0_f64 * t9701;
    let t14327 = 2.0_f64 / 9.0_f64 * t13746;
    let t14329 = t13753 / 9.0_f64;
    (t14295, t14317, t14318, t14327, t14329)
}
