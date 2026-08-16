//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1453/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1453(t10851: f64, t10852: f64, t11187: f64, t11188: f64, t11190: f64, t11259: f64, t11260: f64, t11618: f64, t31692: f64, t4: f64, t8222: f64, t8950: f64, t9416: f64, t9417: f64, t9814: f64) -> f64 {
    let t31707 = t4 * t31692 + 2.0_f64 * t10851 + 2.0_f64 * t10852 + 2.0_f64 * t11187 + 4.0_f64 * t11188 + 2.0_f64 * t11190 + 2.0_f64 * t11259 + 2.0_f64 * t11260 + 2.0_f64 * t11618 + 2.0_f64 * t8222 + 2.0_f64 * t8950 + 2.0_f64 * t9416 + 4.0_f64 * t9417 + 2.0_f64 * t9814;
    t31707
}
