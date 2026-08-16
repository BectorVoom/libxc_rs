//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 598/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk598(t6389: f64, t950: f64, t931: f64, t2988: f64, t6365: f64, t2986: f64, t2992: f64, t4612: f64, t6328: f64, t6332: f64, t6336: f64, t274: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6390 = t6389 * t950;
    let t6392 = 1.0_f64 * t931 * t6390;
    let t6393 = t6365 * t2988;
    let t6395 = 0.16081824322151104822e2_f64 * t2986 * t6393;
    let t6400 = t2992 + 0.61805555555555555556e-2_f64 * t4612 - 0.61805555555555555555e-2_f64 * t6328 + 0.18541666666666666667e-1_f64 * t6332 - 0.92708333333333333333e-2_f64 * t6336;
    let t6401 = t6400 * t274;
    (t6390, t6392, t6393, t6395, t6400, t6401)
}
