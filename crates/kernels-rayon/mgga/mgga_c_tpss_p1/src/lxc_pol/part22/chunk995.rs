//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 995/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk995(t10695: f64, t681: f64, t3589: f64, t680: f64, t682: f64, t2345: f64, t3557: f64, t10557: f64, t10559: f64, t10561: f64, t10566: f64, t10568: f64, t10686: f64, t10688: f64, t10692: f64, t10693: f64, t10694: f64, t8126: f64, t8222: f64) -> (f64, f64, f64, f64) {
    let t10697 = 4.0_f64 * t681 * t10695;
    let t10698 = t680 * t3589;
    let t10700 = 8.0_f64 * t10698 * t682;
    let t10701 = t3557 * t2345;
    let t10702 = 0.11696447245269292414e1_f64 * t10701;
    let t10703 = t10557 - t8126 - t10559 - t10561 + t10566 + t10568 - t10686 + t10688 + t10692 - t10693 + t10694 + t10697 + t10700 + t10702 + t8222;
    (t10697, t10700, t10702, t10703)
}
