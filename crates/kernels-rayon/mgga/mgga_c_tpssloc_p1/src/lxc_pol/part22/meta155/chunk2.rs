//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 966/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk966(t1021: f64, t248: f64, t4650: f64, t1020: f64, t1025: f64, t1041: f64, t1046: f64, t1618: f64, t1622: f64, t3104: f64, t3109: f64, t3114: f64, t3117: f64, t3140: f64, t3156: f64, t3160: f64, t3163: f64, t378: f64, t4617: f64, t4622: f64, t4625: f64, t4631: f64, t4636: f64, t4641: f64, t4644: f64) -> (f64, f64) {
    let t4652 = t248 * t1021 * t4650;
    let t4656 = t3104 / 4608.0_f64 + t4617 * t378 / 3072.0_f64 + t3140 / 864.0_f64 + t3156 / 4608.0_f64 - t4622 * t378 / 576.0_f64 + t4625 / 4608.0_f64 - t3109 * t1618 / 576.0_f64 + t4631 / 4608.0_f64 + t3117 * t1622 / 4608.0_f64 + t1041 * t4636 / 4608.0_f64 + t4641 * t1025 / 3072.0_f64 + t4644 * t1046 / 4608.0_f64 + t3114 * t1618 / 3072.0_f64 + t1020 * t4652 / 3072.0_f64 - t3160 - t3163 / 108.0_f64;
    (t4652, t4656)
}
