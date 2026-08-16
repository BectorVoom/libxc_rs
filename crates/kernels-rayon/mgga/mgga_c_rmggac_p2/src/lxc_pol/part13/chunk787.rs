//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 787/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk787(t1993: f64, t7920: f64, t1997: f64, t7335: f64, t7927: f64, t16156: f64, t7742: f64, t7380: f64, t5542: f64, t7546: f64, t674: f64, t7269: f64, t7508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36520 = t1993 * t7920;
    let t36521 = t36520 * t1997;
    let t36527 = t7335 * t7927;
    let t36533 = t16156 * t7742;
    let t36535 = t16156 * t7380;
    let t36541 = t7546 * t5542;
    let t36542 = t36541 * t674;
    let t36590 = t7508 * t7269;
    (t36520, t36521, t36527, t36533, t36535, t36541, t36542, t36590)
}
