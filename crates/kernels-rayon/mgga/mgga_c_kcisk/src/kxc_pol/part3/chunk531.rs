//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 531/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk531(t1580: f64, t4384: f64, t1581: f64, t3283: f64, t1312: f64, t3532: f64, t539: f64, t3278: f64, t3952: f64, t1567: f64, t1308: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4385 = t1580 * t4384;
    let t4387 = t1581 * t3283;
    let t4388 = t1312 * t4387;
    let t4391 = t539 * t3532;
    let t4392 = t4391 * t3278;
    let t4393 = t3952 * t4392;
    let t4396 = t1567 * sigma0;
    let t4397 = t4396 * t1308;
    (t4385, t4387, t4388, t4391, t4392, t4393, t4396, t4397)
}
