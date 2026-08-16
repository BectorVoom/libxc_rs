//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 293/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk293(t920: f64, t943: f64, t924: f64, t935: f64, t940: f64, t947: f64) -> (f64, f64, f64) {
    let t1214 = 0.516475e0_f64 * t920;
    let t1217 = 0.104195e0_f64 * t943;
    let t1219 = 0.3529725e1_f64 * t935 - t1214 - 0.516475e0_f64 * t924 + 0.6311625e0_f64 * t940 - t1217 - 0.104195e0_f64 * t947;
    (t1214, t1217, t1219)
}
