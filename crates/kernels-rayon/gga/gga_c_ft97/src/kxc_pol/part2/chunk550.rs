//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 550/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk550(t3526: f64, t579: f64, t91: f64, t3318: f64, t3335: f64, t1960: f64, t1963: f64, t2124: f64, t3321: f64, t3325: f64, t3328: f64, t3332: f64, t3340: f64, t3345: f64, t3411: f64, t3493: f64) -> (f64, f64) {
    let t3528 = t91 * t579 * t3526;
    let t3530 = t3318 / 27.0_f64;
    let t3535 = t3335 / 9.0_f64;
    let t3539 = -t3493 / 12.0_f64 + t3528 / 6.0_f64 + t2124 + t1960 + t1963 + t3530 - 2.0_f64 / 27.0_f64 * t3321 + t3325 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t3328 - 2.0_f64 / 9.0_f64 * t3332 + t3535 + t3340 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t3345 - t3411 / 3.0_f64;
    (t3528, t3539)
}
