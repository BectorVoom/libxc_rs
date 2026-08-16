//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 250/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk250(t246: f64, t381: f64, t404: f64, t408: f64, t412: f64, t426: f64, t459: f64, t461: f64, t466: f64, t470: f64, t607: f64, t764: f64, t765: f64, t766: f64) -> f64 {
    let t771 = t764 + 0.675260332e-1_f64 * t765 * t766 - 0.285764e-1_f64 * t246 * t607 - t381 - t404 - t408 + t412 - t426 - t459 - t461 + t466 + t470;
    t771
}
