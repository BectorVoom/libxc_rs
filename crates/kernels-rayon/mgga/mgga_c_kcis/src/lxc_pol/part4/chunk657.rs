//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 657/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk657(t1491: f64, t3728: f64, t1499: f64, t1457: f64, t1466: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t3729 = t3728 * t1491;
    let t3731 = t3728 * t1499;
    let t3733 = t1457 * t1466;
    let t3734 = t3733 * sigma2;
    (t3729, t3731, t3733, t3734)
}
