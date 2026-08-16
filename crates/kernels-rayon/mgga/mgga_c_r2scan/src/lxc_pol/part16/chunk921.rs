//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 921/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk921(t58: f64, t875: f64, t423: f64, t122: f64, t597: f64, t10673: f64, t3308: f64, t870: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10674 = t875 * t58;
    let t10675 = t10674 * t423;
    let t10676 = t597 * t122;
    let t10677 = t10675 * t10676;
    let t10678 = t10673 * t10677;
    let t10680 = t870 * t3308;
    (t10674, t10675, t10676, t10677, t10678, t10680)
}
