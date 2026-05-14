//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 364/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk364<F: Float>(t120: F, t1595: F, t72: F, t123: F, t29: F, t532: F, t126: F, t1631: F, t1655: F, t2009: F, t2012: F, t2014: F, t534: F, t139: F, t527: F, t129: F, t39: F) -> (F, F, F, F, F) {
    let t2015 = t1595 * t120;
    let t2016 = t72 * t2015;
    let t2021 = t123 / t532 / t29;
    let t2022 = t1595 * t126;
    let t2030 = -0.11705142615505742e0 * t2009 + 0.23410285231011484e0 * t2012 - 0.26564305359272358183e-2 * t2014 * t2016 + 0.319782988780431561e-1 * t2021 * t2022 - 0.532971647967385935e-1 * t534 * t1655 * t126 + 0.13977476158628290272e-1 * t1631 * t2022;
    let t2031 = t139 * t2030;
    let t2032 = t527 * t2031;
    let t2034 = t129 * t39;
    (t2015, t2021, t2030, t2032, t2034)
}
