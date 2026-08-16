//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 807/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk807(t16502: f64, t8516: f64, t5016: f64, t9000: f64, t1605: f64, t1986: f64, t118: f64, t128: f64, t1494: f64, t209: f64, t1550: f64, t5144: f64, t7778: f64) -> (f64, f64, f64, f64, f64) {
    let t39437 = t8516 * t16502;
    let t39451 = t5016 * t9000;
    let t39490 = t1986 * t1605;
    let t39513 = t1986 * t118 * t128 * t1494 * t209;
    let t39528 = t1550 * t7778 * t5144;
    (t39437, t39451, t39490, t39513, t39528)
}
