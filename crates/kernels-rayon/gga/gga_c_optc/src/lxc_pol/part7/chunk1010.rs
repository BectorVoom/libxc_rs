//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1010/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1010(t1979: f64, t6838: f64, t1986: f64, t6617: f64, t1849: f64) -> (f64, f64, f64) {
    let t22116 = t1979 * t6838;
    let t22117 = 0.22787712934626154593e-2_f64 * t22116;
    let t22118 = t1986 * t6617;
    let t22119 = 0.14035736153892489771e2_f64 * t22118;
    let t22120 = t1849 * t1849;
    (t22117, t22119, t22120)
}
