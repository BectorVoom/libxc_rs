//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1567/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1567<F: Float>(t3603: F, t6622: F, t1284: F, t24698: F, t487: F, t83107: F, t22648: F, t602: F, t1469: F, t1486: F, t72: F, t23042: F, t3915: F, t686: F) -> (F, F, F, F, F, F, F) {
    let t84645 = t3603 * t6622;
    let t84859 = t24698 * t1284;
    let t84952 = t24698 * t487;
    let t84967 = t83107 * t487;
    let t85037 = t22648 * t602;
    let t85161 = t1469 * t1486 * t72;
    let t85475 = t3915 * t23042 * t72 * t686;
    (t84645, t84859, t84952, t84967, t85037, t85161, t85475)
}
