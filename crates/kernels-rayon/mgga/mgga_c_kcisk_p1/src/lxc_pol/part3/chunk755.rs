//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 755/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk755(t11490: f64, t11645: f64, t673: f64, t716: f64, t720: f64, t415: f64, t1333: f64, t5177: f64, t1871: f64, t5174: f64, t1895: f64, t1869: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t11646 = t11490 + t11645;
    let t11647 = t673 * t11646;
    let t11648 = t11647 * t716;
    let t11649 = t11648 * t720;
    let t11650 = t415 * t11649;
    let t11652 = t1333 * t5177;
    let t11658 = t5174 * t1871;
    let t11659 = t11658 * sigma2;
    let t11660 = t11659 * t1895;
    let t11661 = t1869 * t11660;
    (t11646, t11650, t11652, t11658, t11661)
}
