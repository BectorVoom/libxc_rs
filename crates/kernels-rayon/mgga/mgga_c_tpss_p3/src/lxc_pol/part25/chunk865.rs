//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 865/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk865(t7821: f64, t7824: f64, t7827: f64, t7830: f64, t7834: f64, t7836: f64, t7838: f64, t7841: f64, t705: f64, t2271: f64, t697: f64, t164: f64) -> (f64, f64) {
    let t7843 = -0.47063e1_f64 * t7821 + 0.31375333333333333334e1_f64 * t7824 - 0.36604555555555555556e1_f64 * t7827 - 0.16068111111111111111e1_f64 * t7830 + 0.28051666666666666666e0_f64 * t7834 - 0.56103333333333333332e0_f64 * t7836 - 0.6545388888888888889e0_f64 * t7838 - 0.46308888888888888888e0_f64 * t7841;
    let t7844 = t7843 * t705;
    let t7848 = 1.0_f64 / t2271 / t697;
    let t7849 = t164 * t7848;
    (t7844, t7849)
}
