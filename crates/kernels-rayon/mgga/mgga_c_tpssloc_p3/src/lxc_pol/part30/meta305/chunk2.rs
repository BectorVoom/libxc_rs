//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1328/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1328(t10475: f64, t10478: f64, t10472: f64, t3131: f64, t360: f64, t376: f64, t676: f64, t1023: f64, t248: f64, t1020: f64, t2928: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10482 = t3131 * t360;
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10523 = 1.0_f64 / t2928 / t320;
    (t10480, t10482, t10508, t10510, t10511, t10523)
}
