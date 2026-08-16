//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 322/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk322(t143: f64, t160: f64, t3539: f64, t1030: f64, t1882: f64, t167: f64, t3408: f64, t574: f64, t1055: f64, t1959: f64, t1962: f64, t2149: f64, t3318: f64, t3321: f64, t3325: f64, t3328: f64, t3332: f64, t3335: f64, t3340: f64, t3345: f64, t3411: f64, t3493: f64, t3528: f64) -> (f64, f64, f64, f64, f64) {
    let t3541 = t143 * t3539 * t160;
    let t3545 = t1882 * t1030;
    let t3548 = t574 * t167 * t3408;
    let t3551 = t1882 * t1055;
    let t3565 = -t3493 / 4.0_f64 + t3528 / 2.0_f64 + t2149 + t1959 / 9.0_f64 + t1962 / 3.0_f64 + t3318 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t3321 + t3325 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t3328 - 2.0_f64 / 3.0_f64 * t3332 + t3335 / 3.0_f64 + t3340 / 3.0_f64 + 2.0_f64 * t3345 - t3411;
    (t3541, t3545, t3548, t3551, t3565)
}
