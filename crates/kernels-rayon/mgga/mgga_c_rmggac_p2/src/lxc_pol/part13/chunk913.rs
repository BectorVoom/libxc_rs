//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 913/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk913(t40138: f64, t7284: f64, t34975: f64, t34976: f64, t571: f64, t7455: f64, t39850: f64, t7229: f64, t4550: f64, t495: f64, t8440: f64, t35039: f64, t39851: f64, t498: f64) -> (f64, f64, f64, f64) {
    let t40139 = t40138 * t7284;
    let t40143 = t34975 * t34976 * t571 * t7455;
    let t40145 = t7229 * t39850;
    let t40149 = t40145 * t34976 * t8440 * t4550 * t495;
    let t40154 = t39851 * t35039 * t8440 * t4550 * t498;
    (t40139, t40143, t40149, t40154)
}
