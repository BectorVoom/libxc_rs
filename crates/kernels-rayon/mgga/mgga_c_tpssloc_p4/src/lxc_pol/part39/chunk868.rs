//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 868/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk868(t5210: f64, t562: f64, t1372: f64, t1807: f64, t1808: f64, t225: f64, t1323: f64, t1834: f64, t1811: f64, t3726: f64, t1307: f64, t1810: f64, t210: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5211 = t5210 * t562;
    let t5213 = t1807 * t1372;
    let t5215 = t1808 * t225;
    let t5217 = t1323 * t1834;
    let t5220 = t3726 * t1811;
    let t5223 = t210 * t1810 * t1307;
    (t5211, t5213, t5215, t5217, t5220, t5223)
}
