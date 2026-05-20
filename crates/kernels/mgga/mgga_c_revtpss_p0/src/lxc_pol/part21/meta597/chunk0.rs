//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2316/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2316<F: Float>(t10578: F, t9863: F, t762: F, t9291: F, t2629: F, t2: F, t588: F, t2576: F, t2565: F, t701: F) -> (F, F, F, F, F, F) {
    let t39438 = t10578 * t9863;
    let t39440 = t9291 * t762;
    let t39442 = F::cast_from(0.67471172535210825684e-1_f64) * t2629 * t39440;
    let t39454 = t2 * t588;
    let t39480 = t2576 * t2576;
    let t39483 = F::new(6.0) * t2565 * t39480 * t701;
    (t39438, t39440, t39442, t39454, t39480, t39483)
}
