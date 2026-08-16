//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2068/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2068(t26921: f64, t7648: f64, t2142: f64, t3552: f64, t26983: f64, t7658: f64, t12627: f64, t7635: f64, t27033: f64, t3801: f64, t12587: f64, t7669: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97422 = t7648 * t26921;
    let t97425 = t3552 * t2142;
    let t97453 = t26983 * t7658;
    let t97475 = t12627 * t7635;
    let t97487 = t27033 * t3801;
    let t97491 = t7669 * t12587;
    (t97422, t97425, t97453, t97475, t97487, t97491)
}
