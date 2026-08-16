//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 949/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk949(t19682: f64, t15972: f64, t12094: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64) -> (f64, f64, f64) {
    let t20523 = 0.17544670867903938621e1_f64 * t19682;
    let t20524 = 3.0_f64 * t15972;
    let t20525 = -t12094 + t9793 + t9797 - t9820 - t9824 - t20523 + t20524 + t12103 - t12105 - t12109 - t12114 + t12116;
    (t20523, t20524, t20525)
}
