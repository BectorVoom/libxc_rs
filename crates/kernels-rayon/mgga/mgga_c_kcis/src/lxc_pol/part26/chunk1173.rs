//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1173/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1173(t29657: f64, t449: f64, t446: f64, t448: f64, t7570: f64, t2233: f64, t447: f64, t6887: f64, t2234: f64, t2272: f64, t6896: f64, t1881: f64, t8141: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29658 = t449 * t29657;
    let t29659 = t446 * t29658;
    let t29660 = t29659 / 16.0_f64;
    let t29662 = t448 * t7570;
    let t29663 = t2233 * t29662;
    let t29664 = t29663 / 16.0_f64;
    let t29665 = t6887 * t447;
    let t29666 = t29665 * t2234;
    let t29667 = t29666 / 8.0_f64;
    let t29668 = t6896 * t2272;
    let t29669 = t446 * t29668;
    let t29670 = t29669 / 16.0_f64;
    let t29671 = t1881 * t8141;
    (t29660, t29664, t29665, t29667, t29670, t29671)
}
