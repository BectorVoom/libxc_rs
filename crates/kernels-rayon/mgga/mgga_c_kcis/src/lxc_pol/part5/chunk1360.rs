//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1360/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1360(t16622: f64, t4291: f64, t6012: f64, t17412: f64, t5932: f64, t2047: f64, t5999: f64, t17504: f64, t576: f64, t5905: f64, t20925: f64, t4261: f64) -> (f64, f64, f64, f64, f64) {
    let t22393 = t16622 * t4291;
    let t22394 = t22393 * t6012;
    let t22396 = t17412 * t5932;
    let t22398 = t5999 * t2047;
    let t22400 = t576 * t17504;
    let t22401 = t22400 * t5905;
    let t22403 = t4261 * t20925;
    (t22394, t22396, t22398, t22401, t22403)
}
