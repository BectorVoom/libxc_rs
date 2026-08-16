//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 509/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk509(t1311: f64, t24: f64, t1248: f64, t3575: f64, t1249: f64, t3579: f64, t3583: f64, t4011: f64, t4015: f64, t4018: f64, t4021: f64, t4039: f64, t4047: f64, t4049: f64, t4055: f64, t4057: f64, t4061: f64, t4063: f64) -> (f64, f64, f64, f64, f64) {
    let t4065 = t24 * t1311;
    let t4067 = t1248 * t4065 * t3575;
    let t4070 = t1248 * t1249 * t3579;
    let t4073 = t1248 * t1249 * t3583;
    let t4075 = -0.9494625e0_f64 * t4039 + 0.1898925e1_f64 * t4047 + t4049 + 0.19931111111111111111e0_f64 * t4011 - 0.19931111111111111111e0_f64 * t4015 + 0.59793333333333333334e0_f64 * t4018 - 0.29896666666666666667e0_f64 * t4021 + 0.15358125e0_f64 * t4055 + 0.3071625e0_f64 * t4057 + t4061 + 0.21908444444444444444e0_f64 * t4063 - 0.5477111111111111111e-1_f64 * t4067 + 0.32862666666666666666e0_f64 * t4070 - 0.16431333333333333333e0_f64 * t4073;
    (t4065, t4067, t4070, t4073, t4075)
}
