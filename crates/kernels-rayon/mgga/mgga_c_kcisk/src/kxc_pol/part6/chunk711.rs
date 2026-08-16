//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 711/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk711(t12658: f64, t12659: f64, t211: f64, t3138: f64, t1001: f64, t213: f64, t3174: f64, t1050: f64, t3253: f64, t1042: f64, t3271: f64, t3137: f64, t974: f64) -> (f64, f64, f64, f64, f64) {
    let t12660 = t12658 * t12659;
    let t12662 = t3138 * t211;
    let t12663 = t213 * t1001;
    let t12664 = t12663 * t3174;
    let t12665 = t12662 * t12664;
    let t12667 = t3253 * t1050;
    let t12669 = t1042 * t3271;
    let t12671 = t974 * t3137;
    (t12660, t12665, t12667, t12669, t12671)
}
