//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 977/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk977(t3751: f64, t456: f64, t544: f64, t3752: f64, t518: f64, t10269: f64, t546: f64, t3943: f64, t478: f64, t463: f64, t1075: f64, t237: f64, t451: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11369 = t3751 * t456 * t544;
    let t11374 = t3752 * t518;
    let t11384 = 0.29201909629629629629e-3_f64 * t10269 * t546;
    let t11388 = 1.0_f64 / t3943 / t478;
    let t11402 = 1.0_f64 / t456 / t463 / 4.0_f64;
    let t11407 = t237 * t1075 * t451;
    (t11369, t11374, t11384, t11388, t11402, t11407)
}
