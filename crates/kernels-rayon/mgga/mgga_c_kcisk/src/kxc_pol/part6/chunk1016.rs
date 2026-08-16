//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1016/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1016(t30558: f64, t3679: f64, t12910: f64, t12884: f64, t30551: f64, t12888: f64, t13064: f64, t3725: f64, t19100: f64, t25590: f64, t25601: f64, t25609: f64, t25696: f64, t25699: f64, t25701: f64, t30569: f64, t30572: f64, t30582: f64, t30585: f64, t30606: f64, t30608: f64, t30610: f64) -> (f64, f64, f64, f64) {
    let t30666 = t30558 * t3679;
    let t30668 = 0.96490945932906628932e2_f64 * t12910 * t30666;
    let t30669 = t12884 * t30551;
    let t30670 = t30669 * t12888;
    let t30673 = t13064 * t30551;
    let t30674 = t30673 * t3725;
    let t30691 = -0.60384999999999999999e0_f64 * t30569 + 0.181155e1_f64 * t30572 - 0.40256666666666666668e0_f64 * t19100 + 0.20128333333333333333e0_f64 * t25590 - 0.60385000000000000001e0_f64 * t25601 + 0.30192500000000000001e0_f64 * t25609 - 0.33114e0_f64 * t25696 + 0.16557e0_f64 * t25699 + 0.5519e-1_f64 * t25701 - 0.82785e-1_f64 * t30582 + 0.49671e0_f64 * t30585 + 0.16504875e0_f64 * t30606 + 0.258925e1_f64 * t30608 - 0.3883875e1_f64 * t30610;
    (t30668, t30670, t30674, t30691)
}
