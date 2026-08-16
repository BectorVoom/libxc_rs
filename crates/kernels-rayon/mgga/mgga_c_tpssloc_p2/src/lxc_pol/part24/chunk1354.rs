//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1354/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1354(t10103: f64, t10116: f64, t1902: f64, t1911: f64, t22975: f64, t23150: f64, t23278: f64, t259: f64, t2597: f64, t2718: f64, t2720: f64, t2743: f64, t6627: f64, t6632: f64, t798: f64, t82255: f64, t82259: f64, t82266: f64, t855: f64, t9584: f64, t9593: f64) -> f64 {
    let t82279 = t9584 * t1902 * t259 - 3.0_f64 * t23278 * t2743 - 0.49348022005446793095e-1_f64 * t82255 + 6.0_f64 * t23278 * t2720 + 0.19190897446562641759e0_f64 * t82259 + 6.0_f64 * t6627 * t10116 + 0.14804406601634037928e0_f64 * t82266 + 2.0_f64 * t855 * t2718 * t1911 * t10103 + 6.0_f64 * t2597 * t22975 + 12.0_f64 * t9593 * t6632 + 3.0_f64 * t798 * t23150 * t259;
    t82279
}
