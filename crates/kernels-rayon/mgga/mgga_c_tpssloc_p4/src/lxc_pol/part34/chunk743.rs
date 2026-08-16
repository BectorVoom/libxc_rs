//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 743/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk743(t10470: f64, t10471: f64, t1013: f64, t363: f64, t3034: f64, t6793: f64, t368: f64, t3131: f64, t360: f64, t376: f64, t676: f64, t2928: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10472 = t10470 * t10471;
    let t10473 = t1013 * t1013;
    let t10474 = 1.0_f64 / t10473;
    let t10475 = t10474 * t363;
    let t10477 = 1.0_f64 / t3034 / t6793;
    let t10478 = t368 * t10477;
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10482 = t3131 * t360;
    let t10508 = t676 * t376;
    let t10523 = 1.0_f64 / t2928 / t320;
    (t10472, t10474, t10477, t10478, t10480, t10482, t10508, t10523)
}
