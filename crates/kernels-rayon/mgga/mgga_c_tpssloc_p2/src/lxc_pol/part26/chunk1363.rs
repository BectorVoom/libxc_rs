//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1363/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1363(t11786: f64, t7345: f64, t24716: f64, t3572: f64, t24736: f64, t3523: f64, t11813: f64, t7338: f64, t3566: f64, t7344: f64, t11801: f64, t11708: f64, t24728: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86120 = t7345 * t11786;
    let t86122 = t24716 * t3572;
    let t86124 = t24736 * t3523;
    let t86126 = t11813 * t7338;
    let t86129 = t3566 * t7344;
    let t86136 = t7345 * t11801;
    let t86140 = t11708 * t24728;
    (t86120, t86122, t86124, t86126, t86129, t86136, t86140)
}
