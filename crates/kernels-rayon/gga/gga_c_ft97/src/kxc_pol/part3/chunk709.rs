//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 709/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk709(t1882: f64, t3548: f64, t3575: f64, t1030: f64, t8232: f64, t167: f64, t2101: f64, t9114: f64, t2179: f64, t582: f64, t3596: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13190 = 2.0_f64 / 9.0_f64 * t1882 * t3548;
    let t13196 = 2.0_f64 / 9.0_f64 * t1882 * t3575;
    let t13201 = t8232 * t1030;
    let t13208 = t2101 * t167;
    let t13212 = t9114 * t167;
    let t13220 = t582 * t2179;
    let t13273 = t5 * t3596;
    (t13190, t13196, t13201, t13208, t13212, t13220, t13273)
}
