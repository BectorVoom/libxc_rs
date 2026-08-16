//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 757/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk757(t1229: f64, t676: f64, t486: f64, t11552: f64, t221: f64, t456: f64, t1176: f64, t3242: f64, t10471: f64, t11715: f64, t11712: f64, t11721: f64, t6739: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11789 = t676 * t1229;
    let t11818 = t676 * t486;
    let t11832 = t221 * t11552;
    let t11834 = 5.0_f64 / 1296.0_f64 * t456 * t11832;
    let t11848 = t1176 * t3242;
    let t11880 = t10471 * t11715;
    let t11881 = t11712 * t11880;
    let t11883 = t6739 * t11721;
    (t11789, t11818, t11834, t11848, t11881, t11883)
}
