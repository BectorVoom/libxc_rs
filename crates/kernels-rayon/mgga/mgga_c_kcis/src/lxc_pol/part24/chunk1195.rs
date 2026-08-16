//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1195/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1195(t26717: f64, t27895: f64, t46978: f64, t8033: f64, t2173: f64, t27867: f64, t2822: f64, t27870: f64, t15573: f64, t27914: f64, t8041: f64, t7690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t96302 = 0.61836467013888888889e-4_f64 * t27895 * t26717;
    let t96305 = t46978 * t8033;
    let t96306 = t2173 * t96305;
    let t96339 = t2822 * t27867;
    let t96340 = 0.14739506172839506172e-2_f64 * t96339;
    let t96345 = t2822 * t27870;
    let t96356 = t15573 * t27914;
    let t96358 = 0.46336805555555555556e-3_f64 * t2173 * t96356;
    let t96382 = t2173 * t46978 * t8041;
    let t96388 = t7690 * t96305;
    (t96302, t96306, t96339, t96340, t96345, t96356, t96358, t96382, t96388)
}
