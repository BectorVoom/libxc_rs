//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 487/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk487(t221: f64, t5605: f64, t589: f64, t1392: f64, t1475: f64, t1888: f64, t209: f64) -> (f64, f64, f64) {
    let t6165 = t221 * t5605 * t589;
    let t6169 = t221 * t1475 * t1392;
    let t6172 = t1888 * t209;
    (t6165, t6169, t6172)
}
