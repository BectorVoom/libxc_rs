//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 866/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk866(t10699: f64, t28471: f64, t10690: f64, t28341: f64, t4790: f64, t10639: f64, t15989: f64, t16389: f64, t22564: f64, t22575: f64, t22583: f64, t22698: f64, t22705: f64, t22707: f64, t28362: f64, t28379: f64, t28387: f64, t28394: f64, t28404: f64) -> (f64, f64, f64) {
    let t28472 = t28471 * t10699;
    let t28475 = t10690 * t28341;
    let t28476 = t28475 * t4790;
    let t28492 = -0.40256666666666666668e0_f64 * t15989 + 0.247573125e0_f64 * t28362 + 0.258925e1_f64 * t28394 - t10639 - 0.27595e0_f64 * t16389 + 0.5519e-1_f64 * t22698 + 0.20128333333333333333e0_f64 * t22564 - 0.60385000000000000001e0_f64 * t22575 + 0.30192500000000000001e0_f64 * t22583 - 0.33114e0_f64 * t22705 + 0.16557e0_f64 * t22707 + 0.49671e0_f64 * t28404 - 0.60384999999999999999e0_f64 * t28379 + 0.181155e1_f64 * t28387;
    (t28472, t28476, t28492)
}
