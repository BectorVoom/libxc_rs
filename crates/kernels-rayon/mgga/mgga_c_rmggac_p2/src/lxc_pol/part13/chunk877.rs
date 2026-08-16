//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 877/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk877(t7501: f64, t8562: f64, t2139: f64, t27: f64, t4928: f64, t649: f64, t1605: f64, t1986: f64, t7720: f64, t36787: f64, t8571: f64, t35559: f64) -> (f64, f64, f64, f64, f64) {
    let t39482 = t7501 * t8562;
    let t39486 = t2139 * t27 * t649 * t4928;
    let t39490 = t1986 * t1605;
    let t39491 = t7720 * t39490;
    let t39493 = t8571 * t36787;
    let t39495 = t8571 * t35559;
    (t39482, t39486, t39491, t39493, t39495)
}
