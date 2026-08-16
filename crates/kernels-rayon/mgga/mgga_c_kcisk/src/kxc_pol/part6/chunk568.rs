//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 568/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk568(t1341: f64, t7906: f64, t1415: f64, t1411: f64, t1224: f64, t4013: f64, t7736: f64, t1225: f64, t7740: f64, t7744: f64, t4008: f64, t6020: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7907 = t1341 * t7906;
    let t7908 = t1415 * t7907;
    let t7909 = t1411 * t7908;
    let t7914 = t1224 * t4013 * t7736;
    let t7917 = t1224 * t1225 * t7740;
    let t7920 = t1224 * t1225 * t7744;
    let t7922 = t4008 + 0.11872222222222222222e-1_f64 * t6020 - 0.11872222222222222222e-1_f64 * t7914 + 0.35616666666666666666e-1_f64 * t7917 - 0.17808333333333333333e-1_f64 * t7920;
    (t7907, t7908, t7909, t7914, t7917, t7920, t7922)
}
