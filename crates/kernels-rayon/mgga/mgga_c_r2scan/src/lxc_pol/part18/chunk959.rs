//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 959/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk959(t11539: f64, t3270: f64, t3269: f64, t1108: f64, t2449: f64, t1065: f64, t983: f64, t11002: f64, t10663: f64, t3579: f64, t2526: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11540 = t3270 * t11539;
    let t11541 = t3269 * t11540;
    let t11542 = t11541 / 4.0_f64;
    let t11543 = t2449 * t1108;
    let t11544 = t1065 * t983;
    let t11545 = t11002 * t11544;
    let t11546 = t3269 * t11545;
    let t11547 = 5.0_f64 / 16.0_f64 * t11546;
    let t11548 = t3579 * t10663;
    let t11549 = t11548 / 4.0_f64;
    let t11550 = t797 * t2526;
    (t11540, t11541, t11542, t11543, t11545, t11546, t11547, t11548, t11549, t11550)
}
