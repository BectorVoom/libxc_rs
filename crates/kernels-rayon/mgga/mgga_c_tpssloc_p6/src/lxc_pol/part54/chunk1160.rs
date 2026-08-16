//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1160/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1160(t31419: f64, t6553: f64, t1880: f64, t225: f64, t8544: f64, t6547: f64, t8548: f64, t25168: f64, t2597: f64, t2713: f64, t30673: f64, t30748: f64, t31407: f64, t31409: f64, t31416: f64, t6663: f64, t7087: f64, t855: f64, t8553: f64, t866: f64) -> (f64, f64, f64, f64) {
    let t31420 = t6553 * t31419;
    let t31421 = t1880 * t31420;
    let t31423 = t8544 * t225;
    let t31425 = t6547 * t8548;
    let t31426 = 0.19190897446562641759e-1_f64 * t31425;
    let t31427 = -t30673 - t7087 * t6663 + t31407 + 2.0_f64 * t855 * t31409 + 2.0_f64 * t2597 * t8553 + 2.0_f64 * t2713 * t8553 - 6.0_f64 * t25168 * t31416 - 0.82246703342411321825e-2_f64 * t31421 - t31423 * t866 + t30748 + t31426;
    (t31420, t31423, t31426, t31427)
}
