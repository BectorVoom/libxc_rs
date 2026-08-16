//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 797/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk797(t1507: f64, t456: f64, t3393: f64, t4232: f64, t238: f64, t4239: f64, t86: f64, t4236: f64, t4222: f64, t1523: f64, t318: f64, t334: f64, t565: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12361 = t1507 * t456;
    let t12381 = t3393 * t4232;
    let t12390 = t86 * t238 * t4239;
    let t12392 = t3393 * t4236;
    let t12394 = t3393 * t4222;
    let t12397 = t86 * t318 * t1523;
    let t12401 = 0.11791604938271604938e-1_f64 * t86 * t334 * t565;
    (t12361, t12381, t12390, t12392, t12394, t12397, t12401)
}
