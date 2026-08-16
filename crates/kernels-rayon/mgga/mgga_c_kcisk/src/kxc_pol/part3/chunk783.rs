//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 783/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk783(t10552: f64, t1685: f64, t1676: f64, t4753: f64, t1670: f64, t4787: f64, t10690: f64, t591: f64, t4790: f64, t10681: f64, t10696: f64, t10699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12087 = t10552 * t1685;
    let t12090 = t4753 * t1676;
    let t12095 = t1670 * t4787;
    let t12098 = t591 * t10690;
    let t12099 = t10552 * t4790;
    let t12102 = t10681 * t1685;
    let t12105 = t591 * t10696;
    let t12106 = t10552 * t10699;
    (t12087, t12090, t12095, t12098, t12099, t12102, t12105, t12106)
}
