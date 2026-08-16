//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1156/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1156(t17185: f64, t2367: f64, t913: f64, t10959: f64, t17134: f64, t2812: f64, t17064: f64, t930: f64, t16988: f64, t7433: f64, t8127: f64, t8129: f64) -> (f64, f64, f64, f64, f64) {
    let t51733 = t913 * t2367 * t17185;
    let t51736 = t2812 * t10959 * t17134;
    let t51743 = t930 * t2367 * t17064;
    let t51745 = t7433 * t16988;
    let t51747 = t8127 * t51745 * t8129;
    (t51733, t51736, t51743, t51745, t51747)
}
