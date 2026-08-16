//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 234/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk234(t954: f64, t957: f64, t960: f64, t964: f64, t966: f64, t969: f64, t377: f64, t364: f64) -> (f64, f64, f64) {
    let t1101 = -0.42198333333333333333e0_f64 * t954 + 0.84396666666666666666e0_f64 * t957 + 0.39862222222222222223e0_f64 * t960 + 0.68258333333333333333e-1_f64 * t964 + 0.13651666666666666667e0_f64 * t966 + 0.13692777777777777778e0_f64 * t969;
    let t1102 = t1101 * t377;
    let t1104 = 1.0_f64 * t364 * t1102;
    (t1101, t1102, t1104)
}
