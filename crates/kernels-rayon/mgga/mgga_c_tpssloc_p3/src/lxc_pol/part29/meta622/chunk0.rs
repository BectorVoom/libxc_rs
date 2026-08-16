//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2064/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2064(t11801: f64, t7345: f64, t11708: f64, t24728: f64, t11713: f64, t11715: f64, t11717: f64, t2131: f64, t82985: f64, t24727: f64, t24732: f64, t7337: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86136 = t7345 * t11801;
    let t86140 = t11708 * t24728;
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    let t86154 = t2131 * t82985;
    let t86164 = t11713 * t24727 * t11717;
    let t86167 = t11708 * t24732;
    let t86171 = t11713 * t7337 * t11717;
    (t86136, t86140, t86146, t86154, t86164, t86167, t86171)
}
