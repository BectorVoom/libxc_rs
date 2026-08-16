//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 469/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk469(t334: f64, t3688: f64, t1197: f64, t45: f64, t1202: f64, t330: f64, t1210: f64) -> (f64, f64, f64, f64) {
    let t3689 = t3688 * t334;
    let t3692 = t45 * t1197;
    let t3695 = t1202 * t330;
    let t3696 = 1.0_f64 / t3695;
    let t3697 = t1210 * t1210;
    (t3689, t3692, t3696, t3697)
}
