//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 937/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk937(t2604: f64, t8997: f64, t2134: f64, t27: f64, t4895: f64, t649: f64, t6355: f64, t7810: f64, t2344: f64, t35674: f64, t36391: f64, t9222: f64) -> (f64, f64, f64, f64, f64) {
    let t40578 = t2604 * t8997;
    let t40607 = t2134 * t27 * t649 * t4895;
    let t40610 = t6355 * t7810;
    let t40614 = t35674 * t2344;
    let t40619 = t9222 * t36391;
    (t40578, t40607, t40610, t40614, t40619)
}
