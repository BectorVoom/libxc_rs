//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 724/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk724(t326: f64, t4616: f64, t570: f64, t833: f64, t866: f64, t128: f64, t25525: f64, t338: f64, t3839: f64, t793: f64, t874: f64, t551: f64, t876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27055 = t326 * t4616;
    let t27059 = t570 * t833;
    let t27075 = t570 * t866;
    let t27091 = t25525 * t128;
    let t27094 = t3839 * t338;
    let t27101 = t793 * t874;
    let t27102 = t551 * t876;
    (t27055, t27059, t27075, t27091, t27094, t27101, t27102)
}
