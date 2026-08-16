//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 841/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk841(t1055: f64, t8232: f64, t1882: f64, t3548: f64, t1060: f64, t1986: f64, t2185: f64, t3575: f64, t167: f64, t358: f64, t569: f64, t1030: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13187 = t8232 * t1055;
    let t13190 = 2.0_f64 / 9.0_f64 * t1882 * t3548;
    let t13192 = t2185 * t1060 * t1986;
    let t13196 = 2.0_f64 / 9.0_f64 * t1882 * t3575;
    let t13198 = t569 * t167 * t358;
    let t13201 = t8232 * t1030;
    (t13187, t13190, t13192, t13196, t13198, t13201)
}
