//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 832/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk832(t220: f64, t9817: f64, t124: f64, t1398: f64, t3938: f64, t9816: f64, t1410: f64, t3934: f64, t9757: f64, t9762: f64, t9766: f64, t9771: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t9796: f64, t9799: f64, t9804: f64, t9807: f64, t9812: f64) -> (f64, f64) {
    let t9818 = t9817 * t220;
    let t9819 = t124 * t1398;
    let t9821 = t9818 * t9819 * t3938;
    let t9822 = t9816 * t9821;
    let t9824 = -0.85748036236139473944e-3_f64 * t1410 * t9757 - 0.15246000842785598468e-3_f64 * t9762 + 0.16262400898971305032e-2_f64 * t9766 + 0.21437009059034868486e-4_f64 * t9771 - 0.22866142996303859718e-3_f64 * t9776 - 0.68026775414003982663e-1_f64 * t9780 - t9786 - t9791 - 0.13553694749236397037e-4_f64 * t9796 - 0.5421477899694558815e-4_f64 * t9799 + t9804 + 0.25724410870841842183e-2_f64 * t3934 * t9807 + 0.25724410870841842183e-2_f64 * t3934 * t9812 + 0.30492001685571196935e-3_f64 * t9822;
    (t9821, t9824)
}
