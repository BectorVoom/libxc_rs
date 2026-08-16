//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 896/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk896(t1605: f64, t1986: f64, t7720: f64, t36787: f64, t8571: f64, t35559: f64, t35018: f64, t36740: f64, t9222: f64, t8817: f64, t942: f64, t290: f64, t9030: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39490 = t1986 * t1605;
    let t39491 = t7720 * t39490;
    let t39493 = t8571 * t36787;
    let t39495 = t8571 * t35559;
    let t39497 = t8571 * t35018;
    let t39499 = t9222 * t36740;
    let t39506 = 0.4726e1_f64 * t942 * t8817;
    let t39507 = t290 * t9030;
    (t39491, t39493, t39495, t39497, t39499, t39506, t39507)
}
