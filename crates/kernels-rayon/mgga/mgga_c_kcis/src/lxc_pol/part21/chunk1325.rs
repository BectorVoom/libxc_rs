//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1325/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1325(t26686: f64, t3040: f64, t4781: f64, t14382: f64, t3200: f64, t95911: f64, t2173: f64, t46978: f64, t8041: f64, t7690: f64, t96305: f64, t14654: f64, t3489: f64) -> (f64, f64, f64, f64, f64) {
    let t96372 = t26686 * t4781 * t3040;
    let t96379 = t3200 * t95911 * t14382;
    let t96382 = t2173 * t46978 * t8041;
    let t96388 = t7690 * t96305;
    let t96391 = t14654 * t3489;
    (t96372, t96379, t96382, t96388, t96391)
}
