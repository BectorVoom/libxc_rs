//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 470/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk470(t1345: f64, t1895: f64, t1909: f64, t1912: f64, t1921: f64, t45: f64) -> f64 {
    let t1924 = -t1895 + t1909 + 0.19751789702565206229e-1_f64 * t45 * t1912 - 0.58482233974552040708e0_f64 * t1345 * t1921;
    t1924
}
