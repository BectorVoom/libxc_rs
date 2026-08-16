//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 735/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk735(t24: f64, t1655: f64, t5106: f64, t5107: f64, t5110: f64, t5113: f64, t91: f64, t5104: f64, t98: f64, zeta_threshold: f64) -> f64 {
    let t90 = t24 <= zeta_threshold;
    let t5117 = piecewise3(t90, 0.0_f64, -8.0_f64 / 27.0_f64 * t5106 * t5107 + 4.0_f64 / 3.0_f64 * t5110 * t1655 + 4.0_f64 / 3.0_f64 * t91 * t5113);
    let t5119 = (t5104 + t5117) * t98;
    t5119
}
