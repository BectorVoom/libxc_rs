//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1299/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1299(t2434: f64, t2496: f64, t2629: f64, t676: f64, t9419: f64, t762: f64, t9291: f64, t2: f64, t588: f64, t2576: f64, t2565: f64, t701: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39427 = t2434 * t2496;
    let t39429 = 0.12842595503380418954e1_f64 * t2629 * t39427;
    let t39430 = t676 * t9419;
    let t39432 = 0.38527786510141256862e1_f64 * t2629 * t39430;
    let t39440 = t9291 * t762;
    let t39442 = 0.67471172535210825684e-1_f64 * t2629 * t39440;
    let t39454 = t2 * t588;
    let t39480 = t2576 * t2576;
    let t39483 = 6.0_f64 * t2565 * t39480 * t701;
    (t39427, t39429, t39430, t39432, t39440, t39442, t39454, t39480, t39483)
}
