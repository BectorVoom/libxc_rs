//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1122/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1122(t1065: f64, t2847: f64, t11002: f64, t3274: f64, t5086: f64, t97: f64, t10935: f64, t2813: f64, t3446: f64, t3261: f64, t498: f64, t10648: f64, t10971: f64, t11564: f64) -> (f64, f64, f64, f64, f64) {
    let t40589 = t1065 * t2847;
    let t40590 = t11002 * t40589;
    let t40594 = t97 * t3274 * t5086;
    let t40603 = t3446 * t10935 * t2813;
    let t40604 = 0.19211284388664477842e-2_f64 * t40603;
    let t40630 = t97 * t3261 * t498;
    let t40642 = t10648 * t10971 * t11564;
    (t40590, t40594, t40604, t40630, t40642)
}
