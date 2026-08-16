//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3117/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3117(t1157: f64, t1164: f64, t18785: f64, t3375: f64, t18279: f64, t3378: f64, t1147: f64, t1156: f64, t64425: f64, t15225: f64, t51819: f64, t64482: f64) -> (f64, f64, f64, f64) {
    let t64489 = 0.23392894490538584828e1_f64 * t1164 * t3375 * t18785 * t1157;
    let t64492 = 0.14035736694323150897e2_f64 * t1164 * t18279 * t3378;
    let t64496 = 0.5848223622634646207e0_f64 * t1164 * t1147 * t64425 * t1156;
    let t64499 = 0.41016075432865626631e4_f64 * t51819 * t15225 * t64482;
    (t64489, t64492, t64496, t64499)
}
