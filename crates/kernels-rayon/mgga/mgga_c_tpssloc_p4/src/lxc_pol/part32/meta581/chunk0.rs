//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1963/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1963(t2123: f64, t6146: f64, t2144: f64, t6150: f64, t1720: f64, t8054: f64, t5971: f64, t7286: f64, t24595: f64, t27426: f64, t8002: f64, t2121: f64, t2124: f64, t27755: f64, t27770: f64, t29671: f64, t29674: f64, t29678: f64, t498: f64, t7283: f64, t7999: f64, t8011: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29682 = t6146 * t2123;
    let t29685 = t6150 * t2144;
    let t29687 = t1720 * t8054;
    let t29690 = t7286 * t5971;
    let t29691 = t24595 * t29690;
    let t29694 = t27426 * t8002;
    let t29699 = 0.82246703342411321825e-2_f64 * t2121 * t29671 - 0.16449340668482264365e-1_f64 * t7283 * t29674 - 0.54831135561607547884e-2_f64 * t27755 + 0.80418998823691070228e-1_f64 * t29678 * t2124 - 0.54831135561607547884e-2_f64 * t27770 - 0.82246703342411321825e-2_f64 * t7283 * t29682 + t29685 * t498 + 2.0_f64 * t29687 * t498 + 0.36554090374405031923e-2_f64 * t7283 * t29691 - 0.54831135561607547884e-2_f64 * t7283 * t29694 - 0.43864908449286038306e-1_f64 * t7999 * t8011;
    (t29682, t29685, t29687, t29690, t29691, t29694, t29699)
}
