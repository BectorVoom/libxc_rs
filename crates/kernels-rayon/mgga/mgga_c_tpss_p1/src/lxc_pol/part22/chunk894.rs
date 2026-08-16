//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 894/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk894(t7821: f64, t7824: f64, t7827: f64, t7830: f64, t7834: f64, t7836: f64, t7838: f64, t7841: f64, t676: f64, t657: f64) -> f64 {
    let t7969 = -0.25319e1_f64 * t7821 + 0.16879333333333333333e1_f64 * t7824 - 0.19692555555555555555e1_f64 * t7827 - 0.93011851851851851854e0_f64 * t7830 + 0.13651666666666666667e0_f64 * t7834 - 0.27303333333333333333e0_f64 * t7836 - 0.3185388888888888889e0_f64 * t7838 - 0.36514074074074074075e0_f64 * t7841;
    let t7970 = t7969 * t676;
    let t7972 = 1.0_f64 * t657 * t7970;
    t7972
}
