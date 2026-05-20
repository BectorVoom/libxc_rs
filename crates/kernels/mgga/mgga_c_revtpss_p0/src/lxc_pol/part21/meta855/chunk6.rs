//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3241/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3241<F: Float>(t13190: F, t5023: F, t5505: F, t58477: F, t58479: F, t58481: F, t58591: F, t58688: F, t58690: F, t58692: F, t58695: F, t58700: F, t58703: F) -> F {
    let t60147 = -t13190 * t5023 * t5505 + t58477 + t58479 + t58481 + t58591 - t58688 + t58690 + t58692 - t58695 - t58700 + t58703;
    t60147
}
