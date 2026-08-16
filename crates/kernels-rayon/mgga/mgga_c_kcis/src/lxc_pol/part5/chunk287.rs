//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 287/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk287(t1056: f64, t922: f64, t251: f64, t736: f64, t323: f64, t325: f64, t253: f64) -> (f64, f64, f64, f64) {
    let t1057 = t1056 * t922;
    let t1060 = t736 * t251;
    let t1063 = 0.7925e-3_f64 * t323 * t1060 * t325;
    let t1064 = t251 * t253;
    (t1057, t1060, t1063, t1064)
}
