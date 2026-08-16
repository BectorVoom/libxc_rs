//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 813/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk813(t5507: f64, t9176: f64, t2634: f64, t7624: f64, t4419: f64, t9177: f64, t782: f64, t2041: f64, t9258: f64, t3748: f64, t8090: f64, t8259: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25045 = t5507 * t9176;
    let t25128 = t2634 * t7624;
    let t25130 = t4419 * t9177;
    let t25131 = t782 * t25130;
    let t25153 = t9258 * t2041;
    let t25306 = t3748 * t8090;
    let t25308 = t8259 * sigma0;
    (t25045, t25128, t25131, t25153, t25306, t25308)
}
