//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 186/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk186(t718: f64, t719: f64, t717: f64, t415: f64, t604: f64, t671: f64, t196: f64) -> (f64, f64, f64, f64, f64) {
    let t720 = t718 * t719;
    let t721 = t717 * t720;
    let t722 = t415 * t721;
    let t724 = t604 * t671 + 0.24872916666666666666e-2_f64 * t722;
    let t725 = t604 * t196;
    (t720, t721, t722, t724, t725)
}
