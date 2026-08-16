//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 555/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk555(t2013: f64, t7487: f64, t1297: f64, t20: f64, t2018: f64) -> (f64, f64, f64) {
    let t7488 = t7487 * t2013;
    let t7489 = 0.19211284388664477842e-2_f64 * t7488;
    let t7490 = t1297 * t20;
    let t7491 = t7490 * t2018;
    (t7489, t7490, t7491)
}
