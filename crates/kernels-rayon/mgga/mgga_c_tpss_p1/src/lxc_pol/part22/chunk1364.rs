//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1364/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1364(t3255: f64, t5918: f64, t65551: f64, t65561: f64, t65567: f64, t60696: f64, t60707: f64, t60709: f64, t60713: f64, t62375: f64, t65553: f64, t65555: f64, t65557: f64, t65559: f64) -> (f64, f64) {
    let t67131 = t3255 * t5918;
    let t67138 = 7.0_f64 / 576.0_f64 * t65551;
    let t67143 = 7.0_f64 / 144.0_f64 * t65561;
    let t67148 = 35.0_f64 / 108.0_f64 * t65567;
    let t67149 = t67138 + t65553 / 96.0_f64 - 5.0_f64 / 96.0_f64 * t65555 - t65557 / 48.0_f64 - t62375 + t65559 / 384.0_f64 - t67143 - 7.0_f64 / 576.0_f64 * t60696 - 119.0_f64 / 1728.0_f64 * t60707 + 7.0_f64 / 1152.0_f64 * t60709 + 7.0_f64 / 1152.0_f64 * t60713 - t67148;
    (t67131, t67149)
}
