//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 353/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk353(t645: f64, t1751: f64, t1758: f64, t340: f64, t639: f64, t642: f64, t655: f64, t1720: f64, t397: f64, t662: f64, t656: f64, t122: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t646 = t645 < -0.66725e-1_f64;
    let t1763 = piecewise3(t646, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t1751 * t642 - 10.0_f64 / 27.0_f64 * t340 * t639 * t1758);
    let t1764 = t1763 * sigma2;
    let t1765 = t1764 * t655;
    let t1769 = t397 * t1720 * t662;
    let t1771 = 0.17990788716177317213e-1_f64 * t656 * t1769;
    let t1772 = t655 * t122;
    (t1764, t1765, t1769, t1771, t1772)
}
