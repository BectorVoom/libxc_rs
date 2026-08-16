//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 951/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk951(t28663: f64, t5497: f64, t1775: f64, t5486: f64, t5006: f64, t2642: f64, t7715: f64, t12271: f64, t2364: f64, t25045: f64, t7718: f64, t5491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29811 = t5497 * t28663;
    let t29812 = t1775 * t29811;
    let t29815 = t5486 * t28663;
    let t29816 = t5006 * t29815;
    let t29819 = t7715 * t2642;
    let t29820 = t12271 * t29819;
    let t29821 = t5006 * t29820;
    let t29825 = t25045 * t2364;
    let t29826 = t1775 * t29825;
    let t29831 = t7718 * t2642;
    let t29832 = t5491 * t29831;
    (t29812, t29816, t29819, t29821, t29826, t29832)
}
