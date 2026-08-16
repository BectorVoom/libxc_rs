//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1243/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1243(t3793: f64, t3805: f64, t5301: f64, t3802: f64, t5234: f64, t3788: f64, t836: f64, t1336: f64, t5252: f64, t3777: f64, t5245: f64, t12419: f64, t12420: f64, t5249: f64) -> (f64, f64, f64, f64, f64) {
    let t16391 = t3805 * t5301 * t3793;
    let t16394 = t5234 * t3802;
    let t16397 = t3788 * t836;
    let t16398 = t1336 * t16397;
    let t16400 = 7.0_f64 / 1152.0_f64 * t16398 * t5252;
    let t16401 = t3777 * t5245;
    let t16405 = t12419 * t5249 * t12420;
    (t16391, t16394, t16400, t16401, t16405)
}
