//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 630/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk630(t7899: f64, t8690: f64, t2007: f64, t383: f64, t528: f64, t7977: f64, t7944: f64, t1655: f64, t7946: f64, t7948: f64, t7950: f64, t7952: f64, t7957: f64, t7961: f64, t7964: f64, t7968: f64, t7971: f64, t7975: f64) -> (f64, f64, f64, f64) {
    let t8691 = t8690 * t7899;
    let t8693 = t2007 * t383;
    let t8696 = t528 * t7977;
    let t8698 = 0.44934037037037037036e0_f64 * t7944;
    let t8709 = 0.1760655e0_f64 * t8691 - 0.352131e0_f64 * t8693 * t1655 + 0.234754e0_f64 * t8696 - t8698 - 0.19257444444444444444e0_f64 * t7946 + 0.9628722222222222222e-1_f64 * t7948 - 0.28886166666666666666e0_f64 * t7950 + 0.14443083333333333333e0_f64 * t7952 - 0.1604787037037037037e0_f64 * t7957 + 0.57772333333333333332e0_f64 * t7961 - 0.28886166666666666666e0_f64 * t7964 - 0.86658499999999999998e0_f64 * t7968 + 0.86658499999999999998e0_f64 * t7971 - 0.14443083333333333333e0_f64 * t7975;
    (t8691, t8693, t8696, t8709)
}
