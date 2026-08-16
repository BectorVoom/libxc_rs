//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 984/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk984(t11002: f64, t11544: f64, t3269: f64, t10663: f64, t3579: f64, t2526: f64, t797: f64, t3262: f64, t3263: f64, t2333: f64, t983: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11545 = t11002 * t11544;
    let t11546 = t3269 * t11545;
    let t11547 = 5.0_f64 / 16.0_f64 * t11546;
    let t11548 = t3579 * t10663;
    let t11549 = t11548 / 4.0_f64;
    let t11550 = t797 * t2526;
    let t11552 = t3262 * t3263 * t11550;
    let t11553 = 3.0_f64 / 4.0_f64 * t11552;
    let t11554 = t2333 * t983;
    let t11555 = t11554 * t795;
    (t11545, t11546, t11547, t11548, t11549, t11550, t11552, t11553, t11554, t11555)
}
