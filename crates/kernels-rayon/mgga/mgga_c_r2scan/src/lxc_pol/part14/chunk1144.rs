//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1144/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1144(t10776: f64, t10810: f64, t2574: f64, t3308: f64, t37652: f64, t7379: f64, t3295: f64, t7509: f64, t10708: f64, t10710: f64, t24912: f64, t2183: f64, t37754: f64) -> (f64, f64, f64, f64, f64) {
    let t39911 = t10776 * t10810 * t2574;
    let t39914 = t37652 * t3308 * t7379;
    let t39916 = t3295 * t7509;
    let t39920 = t10708 * t10710 * t24912;
    let t39922 = t2183 * t37754;
    (t39911, t39914, t39916, t39920, t39922)
}
