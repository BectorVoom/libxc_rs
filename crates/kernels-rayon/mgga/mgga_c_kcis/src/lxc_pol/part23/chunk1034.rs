//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1034/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1034(t2237: f64, t27348: f64, t1380: f64, t833: f64, t7909: f64, t3984: f64, t3717: f64, t531: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27349 = t2237 * t27348;
    let t27351 = t833 * t1380;
    let t27352 = t7909 * t27351;
    let t27353 = t3984 * t27352;
    let t27356 = t3717 * t531;
    let t27357 = t833 * t1385;
    (t27349, t27351, t27352, t27353, t27356, t27357)
}
