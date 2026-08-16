//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1727/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1727(t23069: f64, t805: f64, t243: f64, t598: f64, t213: f64, t6584: f64, t6604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23070 = t23069 * t805;
    let t23075 = t243 * t243;
    let t23076 = 1.0_f64 / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23083 = t6584 * t6604;
    (t23070, t23075, t23076, t23077, t23078, t23083)
}
