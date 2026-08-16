//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1364/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1364(t11713: f64, t11715: f64, t11717: f64, t24649: f64, t24658: f64, t2131: f64, t82985: f64, t7325: f64, t10469: f64, t1209: f64, t478: f64, t11720: f64, t3032: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t86146 = t11713 * t11715 * sigma2 * t11717;
    let t86149 = t24658 * t24649;
    let t86154 = t2131 * t82985;
    let t86155 = t86154 * t7325;
    let t86157 = t10469 * t1209 * t478;
    let t86158 = t11720 * t3032;
    (t86146, t86149, t86155, t86157, t86158)
}
