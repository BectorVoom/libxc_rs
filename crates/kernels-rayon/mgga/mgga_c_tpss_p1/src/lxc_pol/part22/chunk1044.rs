//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1044/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1044(t10980: f64, t10986: f64, t11004: f64, t11010: f64, t11015: f64, t11020: f64, t11056: f64, t11059: f64, t11062: f64, t11065: f64, t11068: f64, t11309: f64, t11321: f64, t11328: f64, t11345: f64, t8605: f64, t8607: f64, t8616: f64, t8618: f64, t8627: f64, t8629: f64, t8631: f64) -> f64 {
    let t11347 = 0.17215833333333333333e0_f64 * t8605 + 0.11477222222222222222e0_f64 * t8607 - 0.45908888888888888888e0_f64 * t8616 - 0.34431666666666666666e0_f64 * t8618 - 0.23154444444444444444e0_f64 * t8627 + 0.69463333333333333333e-1_f64 * t8629 + 0.23154444444444444444e-1_f64 * t8631 - 0.22954444444444444444e0_f64 * t10980 + t11309 - 0.516475e0_f64 * t10986 + t11321 - 0.69463333333333333334e-1_f64 * t11056 - 0.34731666666666666667e-1_f64 * t11059 - 0.46308888888888888889e-1_f64 * t11062 + 0.41678e0_f64 * t11065 + 0.20839e0_f64 * t11068 + t11328 - 0.68863333333333333333e0_f64 * t11004 - 0.57386111111111111112e0_f64 * t11010 + 0.20659e1_f64 * t11015 - 0.68863333333333333334e0_f64 * t11020 + t11345;
    t11347
}
