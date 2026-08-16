//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1043/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1043(t102801: f64, t1992: f64, t22897: f64, t3792: f64, t114104: f64, t114119: f64, t122507: f64, t122533: f64, t122535: f64, t127402: f64, t127403: f64, t127404: f64, t127408: f64, t127412: f64) -> f64 {
    let t128880 = t1992 * t22897 * t102801 * t3792;
    let t128882 = -0.82246703342411321824e-2_f64 * t122507 + t114104 + t127402 - t127403 - t127404 - t127408 + t127412 + 0.16449340668482264365e-1_f64 * t122533 + 0.76763589786250567036e-1_f64 * t122535 + 0.16449340668482264365e-1_f64 * t128880 + t114119;
    t128882
}
