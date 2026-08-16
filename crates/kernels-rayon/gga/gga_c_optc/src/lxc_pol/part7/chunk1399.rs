//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1399/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1399(t26160: f64, t26163: f64, t26168: f64, t26170: f64, t26173: f64, t27346: f64, t27843: f64, t27846: f64, t27849: f64, t27856: f64, t27858: f64, t27860: f64, t3245: f64, t4281: f64, t4290: f64) -> f64 {
    let t27862 = -t26160 + t26163 + 2.0_f64 / 9.0_f64 * t27843 + t26168 + t26170 - 4.0_f64 / 3.0_f64 * t27846 + 8.0_f64 / 9.0_f64 * t27849 - t26173 + 6.0_f64 * t4281 * t3245 * t4290 * t27346 + 56.0_f64 / 81.0_f64 * t27856 + 8.0_f64 / 9.0_f64 * t27858 - 2.0_f64 / 3.0_f64 * t27860;
    t27862
}
