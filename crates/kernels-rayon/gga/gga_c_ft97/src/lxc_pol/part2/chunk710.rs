//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 710/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk710(t11200: f64, t26: f64, t2999: f64, t11167: f64, t11170: f64, t11172: f64, t11177: f64, t11180: f64, t11183: f64, t11186: f64, t11189: f64, t11192: f64, t11195: f64, t11198: f64, t7945: f64, t7946: f64, t7948: f64, t7950: f64, t7952: f64) -> (f64, f64) {
    let t11202 = t26 * t2999 * t11200;
    let t11204 = -t7945 - 8.0_f64 / 27.0_f64 * t7946 + 2.0_f64 / 27.0_f64 * t7948 - 2.0_f64 / 9.0_f64 * t7950 + t7952 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t11167 + t11170 - t11172 + 22.0_f64 / 9.0_f64 * t11177 - 10.0_f64 / 27.0_f64 * t11180 + 4.0_f64 / 3.0_f64 * t11183 - 8.0_f64 / 9.0_f64 * t11186 - 2.0_f64 / 9.0_f64 * t11189 - 2.0_f64 * t11192 + 8.0_f64 / 3.0_f64 * t11195 + 2.0_f64 / 3.0_f64 * t11198 - 2.0_f64 / 3.0_f64 * t11202;
    (t11202, t11204)
}
