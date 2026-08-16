//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 814/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk814(t7647: f64, t7662: f64, t799: f64, t779: f64, t2414: f64, t777: f64, t216: f64, t2374: f64, t798: f64, t231: f64, t2417: f64, t2372: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7663 = t7647 + t7662;
    let t7664 = t7663 * t799;
    let t7666 = 1.0_f64 * t779 * t7664;
    let t7668 = 1.0_f64 / t2414 / t777;
    let t7669 = t216 * t7668;
    let t7670 = t2374 * t798;
    let t7672 = 1.0_f64 / t2417 / t231;
    let t7673 = t7670 * t7672;
    let t7675 = 0.51725014705706168417e3_f64 * t7669 * t7673;
    let t7676 = t774 * t2372;
    (t7663, t7664, t7666, t7668, t7669, t7670, t7672, t7673, t7675, t7676)
}
