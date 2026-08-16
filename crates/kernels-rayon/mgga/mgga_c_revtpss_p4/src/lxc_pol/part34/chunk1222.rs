//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1222/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1222(t25953: f64, t27899: f64, t27928: f64, t9775: f64, t5622: f64, t94443: f64, t5609: f64, t7028: f64, t9845: f64, t1889: f64, t94545: f64, t5665: f64, t94497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98104 = t27899 * t25953;
    let t98141 = t9775 * t27928;
    let t98148 = t94443 * t5622;
    let t98161 = t9845 * t7028 * t5609;
    let t98165 = t94545 * t1889;
    let t98174 = t94497 * t5665;
    (t98104, t98141, t98148, t98161, t98165, t98174)
}
