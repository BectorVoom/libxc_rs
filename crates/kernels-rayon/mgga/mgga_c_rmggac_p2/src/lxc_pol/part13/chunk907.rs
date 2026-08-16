//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 907/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk907(t7508: f64, t8568: f64, t27102: f64, t3351: f64, t3352: f64, t875: f64, t2286: f64, t34881: f64, t7424: f64, t8571: f64, t2298: f64, t24363: f64) -> (f64, f64, f64, f64, f64) {
    let t40039 = t7508 * t8568;
    let t40043 = t3351 * t3352 * t875 * t27102;
    let t40045 = t34881 * t2286;
    let t40047 = t8571 * t7424;
    let t40050 = t24363 * t2298;
    (t40039, t40043, t40045, t40047, t40050)
}
