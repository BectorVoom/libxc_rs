//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1003/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1003(t1123: f64, t287: f64, t9562: f64, t302: f64, t3645: f64, t3685: f64, t2105: f64, t3515: f64, t1137: f64, t1066: f64, t2030: f64, t3679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11028 = t1123 * t287;
    let t11029 = t9562 * t11028;
    let t11030 = t302 * t11029;
    let t11033 = t3685 * t3645;
    let t11034 = t2105 * t11033;
    let t11037 = t287 * t3515;
    let t11038 = t1137 * t11037;
    let t11039 = t2105 * t11038;
    let t11042 = t2030 * t1066;
    let t11043 = t3679 * t11042;
    (t11028, t11029, t11030, t11033, t11034, t11037, t11038, t11039, t11043)
}
