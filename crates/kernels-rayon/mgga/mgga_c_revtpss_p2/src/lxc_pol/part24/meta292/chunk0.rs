//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1075/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1075(t1179: f64, t6513: f64, t1160: f64, t6481: f64, t3479: f64, t6502: f64, t12472: f64, t6486: f64, t1130: f64, t6433: f64, t3435: f64, t6470: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20526 = t6513 * t1179;
    let t20542 = t6481 * t1160;
    let t20618 = t6502 * t3479;
    let t20625 = t6486 * t12472;
    let t20629 = t6433 * t1130;
    let t20644 = t6470 * t3435;
    (t20526, t20542, t20618, t20625, t20629, t20644)
}
