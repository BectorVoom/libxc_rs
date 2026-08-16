//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 801/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk801(t1218: f64, t1232: f64, t2134: f64, t2136: f64, t24704: f64, t24706: f64, t24712: f64, t24716: f64, t24723: f64, t24729: f64, t24733: f64, t24736: f64, t24741: f64, t24747: f64, t24749: f64, t24752: f64, t24754: f64, t3496: f64, t3511: f64, t3518: f64, t3527: f64, t3531: f64, t3580: f64, t7339: f64, t7345: f64) -> f64 {
    let t24756 = -t24704 - 0.10093189023535097714e-3_f64 * t2134 * t24706 - t7345 * t3527 / 2304.0_f64 - 0.20186378047070195428e-3_f64 * t24712 - t7345 * t3531 / 1152.0_f64 + t24716 * t1218 / 768.0_f64 + 0.20186378047070195428e-3_f64 * t24723 + t7339 * t3496 / 1536.0_f64 + t24729 * t3511 / 768.0_f64 - t24733 * t3518 / 1536.0_f64 - t24736 * t1232 / 1152.0_f64 - t24741 * t3580 / 1152.0_f64 - 0.20186378047070195428e-3_f64 * t24747 - 0.10093189023535097714e-3_f64 * t24749 * t2136 - t24752 / 1728.0_f64 + t24754 / 1152.0_f64;
    t24756
}
