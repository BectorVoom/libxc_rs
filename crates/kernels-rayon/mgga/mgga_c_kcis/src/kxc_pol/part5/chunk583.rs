//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 583/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk583(t1157: f64, t3393: f64, t1160: f64, t238: f64, t86: f64, t2840: f64, t41: f64, t2844: f64, t339: f64, t1083: f64, t330: f64, t1071: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3394 = t3393 * t1157;
    let t3397 = t86 * t238 * t1160;
    let t3399 = t41 * t2840;
    let t3400 = t339 * t2844;
    let t3405 = t1083 * t330;
    let t3410 = t339 * t1071;
    (t3394, t3397, t3399, t3400, t3405, t3410)
}
