//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 986/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk986(t10783: f64, t1899: f64, t1108: f64, t3604: f64, t1107: f64, t9451: f64, t1096: f64, t3577: f64, t1095: f64, t9422: f64, t1073: f64, t3528: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10785 = 0.48245938496077605201e2_f64 * t1899 * t10783;
    let t10786 = t1108 * t3604;
    let t10789 = t9451 * t1107;
    let t10792 = t1096 * t3577;
    let t10795 = t9422 * t1095;
    let t10800 = t3528 * t1073;
    (t10785, t10786, t10789, t10792, t10795, t10800)
}
