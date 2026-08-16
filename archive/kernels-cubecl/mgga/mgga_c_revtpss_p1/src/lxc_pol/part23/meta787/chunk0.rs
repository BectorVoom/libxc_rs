//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2599/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2599<F: Float>(t18662: F, t41070: F, t686: F, t72: F, t18658: F, t786: F, t789: F, t18796: F, t2465: F, t2470: F, t18811: F, t2435: F) -> (F, F, F, F) {
    let t61348 = t41070 * t18662 * t72 * t686;
    let t61351 = t786 * t18658 * t789;
    let t61355 = t2465 * t18796 * t2470;
    let t61361 = t2435 * t18811;
    (t61348, t61351, t61355, t61361)
}
