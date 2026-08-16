//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 860/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk860(t1659: f64, t28385: f64, t26: f64, t10738: f64, t15989: f64, t16389: f64, t22564: f64, t22575: f64, t22583: f64, t22698: f64, t22705: f64, t22707: f64, t28362: f64, t28379: f64, t28387: f64, t28394: f64) -> (f64, f64) {
    let t28403 = t1659 * t28385;
    let t28404 = t26 * t28403;
    let t28408 = -0.39862222222222222223e0_f64 * t15989 + 0.46074375e0_f64 * t28362 + 0.1898925e1_f64 * t28394 - t10738 - 0.27385555555555555556e0_f64 * t16389 + 0.5477111111111111111e-1_f64 * t22698 + 0.19931111111111111111e0_f64 * t22564 - 0.59793333333333333333e0_f64 * t22575 + 0.29896666666666666667e0_f64 * t22583 - 0.32862666666666666666e0_f64 * t22705 + 0.16431333333333333333e0_f64 * t22707 + 0.49293999999999999999e0_f64 * t28404 - 0.59793333333333333333e0_f64 * t28379 + 0.17938e1_f64 * t28387;
    (t28404, t28408)
}
