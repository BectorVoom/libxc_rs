//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1098/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1098(t26933: f64, t7749: f64, t1165: f64, t1176: f64, t283: f64, t7755: f64, t3190: f64, t3338: f64, t7754: f64, t389: f64, t9568: f64, t3219: f64, t5077: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26934 = t26933 * t7749;
    let t26936 = t1165 * t1176;
    let t26938 = t1165 * t283;
    let t26939 = t26938 * t7755;
    let t26941 = t3338 * t3190;
    let t26942 = t7754 * t26941;
    let t26944 = t9568 * t389;
    let t26946 = t5077 * t3219;
    (t26934, t26936, t26938, t26939, t26941, t26942, t26944, t26946)
}
