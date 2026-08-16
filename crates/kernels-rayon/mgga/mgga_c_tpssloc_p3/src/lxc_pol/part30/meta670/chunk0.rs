//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2099/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2099(t22643: f64, t7691: f64, t81195: f64, t26502: f64, t532: f64, t22573: f64, t7684: f64, t2018: f64, t40611: f64, t2022: f64, t5381: f64, t26509: f64, t580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91548 = t81195 * t22643 * t7691;
    let t91620 = t532 * t26502;
    let t91655 = t7684 * t22573;
    let t91686 = t2018 * t40611;
    let t91813 = 2.0_f64 * t2022 * t5381;
    let t91816 = 2.0_f64 * t26509 * t580;
    (t91548, t91620, t91655, t91686, t91813, t91816)
}
