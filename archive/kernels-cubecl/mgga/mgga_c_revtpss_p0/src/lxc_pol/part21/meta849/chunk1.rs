//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3190/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3190<F: Float>(t58598: F, t58700: F, t58703: F, t58707: F, t58711: F, t58713: F, t58715: F, t58718: F, t58720: F, t58722: F, t58726: F, t57817: F, t57912: F, t58472: F, t58475: F, t58477: F, t58479: F, t58481: F, t58591: F, t58686: F, t58688: F, t58690: F, t58692: F, t58695: F) -> F {
    let t58727 = -t58700 + t58703 - t58707 - t58711 + t58598 - t58713 - t58715 + t58718 - t58720 - t58722 - t58726;
    let t58730 = t57817 + t57912 + t58686 + t58472 + t58475 + t58477 + t58479 + t58481 + t58591 - t58688 + t58690 + t58692 - t58695 + t58727;
    t58730
}
