//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 847/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk847(t36733: f64, t8450: f64, t7478: f64, t7244: f64, t8432: f64, t1614: f64, t2084: f64, t2139: f64, t27: f64, t34884: f64, t9123: f64, t4601: f64, t9008: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42085 = t8450 * t36733;
    let t42086 = t42085 * t7478;
    let t42101 = t7244 * t8432;
    let t42132 = t2139 * t27 * t2084 * t1614;
    let t42144 = t34884 * t9123;
    let t42151 = t4601 * t9008;
    (t42085, t42086, t42101, t42132, t42144, t42151)
}
