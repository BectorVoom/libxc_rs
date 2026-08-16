//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1341/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1341(t10348: f64, t135: f64, t973: f64, t3014: f64, t10263: f64, t10349: f64, t2960: f64, t3011: f64, t340: f64, t343: f64, t42903: f64, t42906: f64, t42909: f64, t42911: f64, t42914: f64, t42916: f64, t42918: f64, t974: f64) -> f64 {
    let t42925 = t973 * t135 * t10348;
    let t42927 = t3014 * t3014;
    let t42933 = -0.12345679012345679012e-2_f64 * t42903 + 0.11111111111111111111e-2_f64 * t42906 - 0.11111111111111111111e-2_f64 * t42909 - 0.59259259259259259257e-2_f64 * t42911 + 0.11111111111111111111e-2_f64 * t42914 + 0.88888888888888888887e-2_f64 * t42916 - 0.32592592592592592592e-1_f64 * t42918 - 0.48888888888888888888e-1_f64 * t10263 * t3011 + 0.88888888888888888888e-2_f64 * t2960 * t10349 - 0.11111111111111111111e-2_f64 * t42925 - 0.83333333333333333332e-3_f64 * t973 * t974 * t340 * t42927 * t343;
    t42933
}
