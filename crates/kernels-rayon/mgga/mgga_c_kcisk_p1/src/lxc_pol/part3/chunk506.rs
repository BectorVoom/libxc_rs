//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 506/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk506(t10: f64, t1337: f64, t1224: f64, t3575: f64, t1225: f64, t3579: f64, t3583: f64, t4008: f64, t4011: f64, t1229: f64, t1233: f64, t1232: f64, t357: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4013 = t10 * t1337;
    let t4015 = t1224 * t4013 * t3575;
    let t4018 = t1224 * t1225 * t3579;
    let t4021 = t1224 * t1225 * t3583;
    let t4023 = t4008 + 0.11872222222222222222e-1_f64 * t4011 - 0.11872222222222222222e-1_f64 * t4015 + 0.35616666666666666666e-1_f64 * t4018 - 0.17808333333333333333e-1_f64 * t4021;
    let t4026 = t1229 * t1233;
    let t4029 = t1232 * t357;
    (t4013, t4015, t4018, t4021, t4023, t4026, t4029)
}
