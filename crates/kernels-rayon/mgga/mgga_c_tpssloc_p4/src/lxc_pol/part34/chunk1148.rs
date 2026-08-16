//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1148/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1148(t28427: f64, t6579: f64, t28419: f64, t22893: f64, t28341: f64, t81640: f64, t23110: f64, t23185: f64, t28418: f64, t23168: f64, t28330: f64, t234: f64, t5631: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98490 = t6579 * t28427;
    let t98505 = t6579 * t28419;
    let t98516 = t81640 * t22893 * t28341;
    let t98549 = t23185 * t23110 * t28418;
    let t98564 = t23168 * t28330;
    let t98598 = t234 * t5631;
    (t98490, t98505, t98516, t98549, t98564, t98598)
}
