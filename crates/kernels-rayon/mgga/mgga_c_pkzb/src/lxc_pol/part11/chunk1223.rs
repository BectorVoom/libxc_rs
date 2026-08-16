//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1223/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1223(t10901: f64, t5802: f64, t683: f64, t10780: f64, t5734: f64, t1855: f64, t2783: f64, t3550: f64, t1084: f64, t9389: f64, t10783: f64, t5771: f64) -> (f64, f64, f64, f64, f64) {
    let t30203 = 0.57895126195293126241e3_f64 * t5802 * t10901 * t683;
    let t30205 = 6.0_f64 * t5734 * t10780;
    let t30208 = 6.0_f64 * t1855 * t2783 * t3550;
    let t30211 = 6.0_f64 * t1855 * t1084 * t9389;
    let t30213 = 0.48245938496077605201e2_f64 * t5771 * t10783;
    (t30203, t30205, t30208, t30211, t30213)
}
