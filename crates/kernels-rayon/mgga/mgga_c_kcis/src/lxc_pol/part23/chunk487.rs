//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 487/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk487(t333: f64, t4051: f64, t4016: f64, t532: f64, t833: f64, t160: f64, t531: f64) -> (f64, f64, f64, f64) {
    let t4053 = 0.16804375e-4_f64 * t333 * t4051;
    let t4054 = 0.23911438650126355246e-1_f64 * t4016;
    let t4055 = t532 * t833;
    let t4059 = t160 * t531;
    (t4053, t4054, t4055, t4059)
}
