//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1277/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1277(t27551: f64, t7327: f64, t135: f64, t24847: f64, t7284: f64, t11713: f64, t11715: f64, t11717: f64, t2131: f64, t82985: f64, t7325: f64, t10469: f64, t1209: f64, t478: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t86077 = t7327 * t27551;
    let t86094 = t24847 * t135 * t7284;
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    let t86154 = t2131 * t82985;
    let t86155 = t86154 * t7325;
    let t86157 = t10469 * t1209 * t478;
    (t86077, t86094, t86146, t86155, t86157)
}
