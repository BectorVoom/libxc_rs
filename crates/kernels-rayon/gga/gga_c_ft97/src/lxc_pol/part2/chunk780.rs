//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 780/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk780(t11167: f64, t11177: f64, t11183: f64, t11192: f64, t11202: f64, t12233: f64, t7946: f64, t7948: f64, t7950: f64, t7952: f64, t8698: f64, t637: f64, t639: f64) -> f64 {
    let t12234 = -0.9628722222222222222e-1_f64 * t7950 + 0.10591594444444444444e1_f64 * t11177 - 0.28886166666666666666e0_f64 * t11202 - t8698 + 0.3209574074074074074e-1_f64 * t7948 - 0.12838296296296296296e0_f64 * t7946 + 0.4814361111111111111e-1_f64 * t7952 + 0.57772333333333333332e0_f64 * t11183 - 0.86658499999999999998e0_f64 * t11192 - 0.6419148148148148148e-1_f64 * t11167 + t12233;
    let t12236 = t637 * t639 * t12234;
    t12236
}
