//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 648/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk648(t262: f64, t8645: f64, t7198: f64, t2347: f64, t352: f64, t7204: f64, t1987: f64, t8571: f64, t5011: f64, t681: f64, t2085: f64, t2373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8646 = t262 * t8645;
    let t8647 = t7198 * t8646;
    let t8649 = t2347 * t352;
    let t8650 = t262 * t8649;
    let t8651 = t7204 * t8650;
    let t8653 = t8571 * t1987;
    let t8655 = t5011 * t681;
    let t8657 = t2373 * t2085;
    (t8646, t8647, t8649, t8650, t8651, t8653, t8655, t8657)
}
