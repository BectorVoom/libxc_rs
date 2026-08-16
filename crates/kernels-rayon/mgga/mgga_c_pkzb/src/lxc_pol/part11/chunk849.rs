//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 849/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk849(t3528: f64, t5511: f64, t667: f64, t2754: f64, t2759: f64, t1861: f64, t3532: f64, t204: f64, t3515: f64, t648: f64) -> (f64, f64, f64, f64, f64) {
    let t9137 = t5511 * t3528;
    let t9138 = t9137 * t667;
    let t9140 = t2754 * t2759;
    let t9142 = t1861 * t3532;
    let t9143 = t9142 * t667;
    let t9148 = t204 * t648 * t3515;
    (t9137, t9138, t9140, t9143, t9148)
}
