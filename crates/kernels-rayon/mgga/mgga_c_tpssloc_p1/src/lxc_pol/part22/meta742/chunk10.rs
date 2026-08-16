//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2462/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2462(t4657: f64, t5872: f64, t1022: f64, t1058: f64, t1060: f64, t1063: f64, t11034: f64, t11059: f64, t11060: f64, t1610: f64, t18086: f64, t18089: f64, t18094: f64, t18129: f64, t21481: f64, t21614: f64, t21626: f64, t21637: f64, t21657: f64, t3186: f64, t3200: f64, t3201: f64, t43553: f64, t43554: f64, t4649: f64, t4669: f64, t4673: f64, t4678: f64, t47853: f64, t5928: f64) -> (f64, f64) {
    let t69996 = t4657 * t5872;
    let t70009 = t1022 * t1058 * t1060 * t21614 - 36.0_f64 * t1022 * t21637 * t43553 * t43554 + 18.0_f64 * t11059 * t11060 * t4649 * t5928 + 6.0_f64 * t21626 * t3186 * t4673 - 3.0_f64 * t3200 * t3201 * t69996 + t1063 * t21481 + 6.0_f64 * t11034 * t21657 + 3.0_f64 * t1610 * t18129 + 3.0_f64 * t18086 * t4678 + 6.0_f64 * t18089 * t4669 + 3.0_f64 * t18094 * t47853;
    (t69996, t70009)
}
