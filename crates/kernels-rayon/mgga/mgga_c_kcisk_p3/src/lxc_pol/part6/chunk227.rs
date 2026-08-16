//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 227/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk227(t151: f64, t167: f64, t175: f64, t60: f64, t852: f64, t945: f64, t955: f64, t972: f64) -> f64 {
    let t974 = -0.11713266981940447749e-2_f64 * t167 * t151 - 0.23426533963880895498e-2_f64 * t945 * t955 - t852 * t175 - t60 * t972;
    t974
}
