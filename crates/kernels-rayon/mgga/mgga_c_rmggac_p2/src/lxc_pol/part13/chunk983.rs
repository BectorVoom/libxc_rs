//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 983/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk983(t27059: f64, t3351: f64, t3352: f64, t515: f64, t2019: f64, t2020: f64, t8858: f64, t2010: f64, t2012: f64, t5757: f64, t4962: f64, t8854: f64) -> (f64, f64, f64, f64, f64) {
    let t41600 = t3351 * t3352 * t515 * t27059;
    let t41604 = t2019 * t2020 * t8858;
    let t41607 = t2010 * t2012 * t5757;
    let t41610 = t2010 * t2012 * t4962;
    let t41613 = t2019 * t2020 * t8854;
    (t41600, t41604, t41607, t41610, t41613)
}
