//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 848/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk848(t6883: f64, t8612: f64, t8511: f64, t9239: f64, t131: f64, t7025: f64, t2240: f64, t1862: f64, t31: f64, t625: f64, t8301: f64, t8515: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31662 = t6883 * t8612;
    let t31663 = 0.19190897446562641759e-1_f64 * t31662;
    let t31675 = t9239 * t8511;
    let t31680 = t7025 * t131;
    let t31681 = t2240 * t31680;
    let t31682 = t1862 * t31;
    let t31687 = t8301 * t625;
    let t31688 = t2240 * t31687;
    let t31690 = 5.0_f64 / 27.0_f64 * t31688 * t8515;
    (t31663, t31675, t31680, t31681, t31682, t31687, t31688, t31690)
}
