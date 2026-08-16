//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 863/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk863(t28408: f64, t28437: f64, t1664: f64, t1645: f64, t10757: f64, t28357: f64, t10755: f64, t10761: f64, t15989: f64, t22564: f64, t22575: f64, t22583: f64, t28371: f64, t28375: f64, t28379: f64, t28383: f64, t28387: f64, t28391: f64) -> (f64, f64, f64) {
    let t28438 = t28408 + t28437;
    let t28439 = t28438 * t1664;
    let t28441 = 1.0_f64 * t1645 * t28439;
    let t28442 = t28357 * t10757;
    let t28444 = 0.51725014705706168417e3_f64 * t10755 * t28442;
    let t28455 = -t10761 - 0.12361111111111111111e-1_f64 * t15989 + 0.61805555555555555556e-2_f64 * t22564 - 0.18541666666666666667e-1_f64 * t22575 + 0.92708333333333333334e-2_f64 * t22583 - 0.10300925925925925926e-1_f64 * t28371 + 0.37083333333333333333e-1_f64 * t28375 - 0.18541666666666666666e-1_f64 * t28379 - 0.55625000000000000001e-1_f64 * t28383 + 0.55625000000000000001e-1_f64 * t28387 - 0.92708333333333333333e-2_f64 * t28391;
    (t28441, t28444, t28455)
}
