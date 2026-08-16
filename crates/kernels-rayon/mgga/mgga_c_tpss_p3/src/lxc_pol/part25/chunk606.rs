//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 606/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk606(t1600: f64, t645: f64, t1342: f64, t2112: f64, t2335: f64, t1398: f64, t823: f64, t198: f64, t205: f64) -> (f64, f64, f64, f64, f64) {
    let t3542 = t1600 * t645;
    let t3546 = 4.0_f64 * t2112 * t1342;
    let t3547 = 4.0_f64 * t2335;
    let t3548 = t1398 * t823;
    let t3552 = t198 * t205;
    (t3542, t3546, t3547, t3548, t3552)
}
