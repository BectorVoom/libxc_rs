//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 885/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk885(t1869: f64, t28775: f64, t15936: f64, t8780: f64, t1800: f64, t2509: f64, t8858: f64, t415: f64, t11197: f64, t23947: f64, t23949: f64, t23951: f64, t28262: f64, t28532: f64, t28758: f64, t28762: f64, t28765: f64, t28768: f64, t671: f64) -> (f64, f64, f64, f64) {
    let t28776 = t1869 * t28775;
    let t28778 = t15936 * t8780;
    let t28779 = t1800 * t28778;
    let t28780 = t1869 * t28779;
    let t28782 = t2509 * t8858;
    let t28783 = t415 * t28782;
    let t28785 = t28532 * t671 + 0.16581944444444444444e-2_f64 * t28758 + 0.73697530864197530861e-2_f64 * t28762 + 0.49745833333333333332e-2_f64 * t28765 + 0.49745833333333333332e-2_f64 * t28768 - 0.43134342e-1_f64 * t11197 * t28262 + 0.66327777777777777776e-2_f64 * t23947 - 0.17687407407407407407e-1_f64 * t23949 - 0.66327777777777777775e-2_f64 * t23951 + 0.39796666666666666665e-1_f64 * t28776 + 0.44218518518518518518e-2_f64 * t28780 + 0.72960555555555555553e-1_f64 * t28783;
    (t28776, t28780, t28783, t28785)
}
