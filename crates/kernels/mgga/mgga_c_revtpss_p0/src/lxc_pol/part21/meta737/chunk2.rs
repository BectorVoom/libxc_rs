//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2590/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2590<F: Float>(t10150: F, t2435: F, t686: F, t72: F, t9651: F, t9680: F, t1358: F, t2439: F, t4066: F, t785: F, t9303: F, t9641: F) -> (F, F, F, F) {
    let t47608 = t2435 * t10150;
    let t47612 = t9680 * t9651 * t72 * t686;
    let t47616 = t2439 * t785 * t4066 * t1358;
    let t47618 = t9303 * t9641;
    (t47608, t47612, t47616, t47618)
}
