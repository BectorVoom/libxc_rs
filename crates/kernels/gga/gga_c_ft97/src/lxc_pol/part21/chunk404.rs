//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 404/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk404<F: Float>(t1013: F, t3355: F, t2007: F, t4441: F, t4466: F, t528: F, t120: F, t72: F, t126: F, t1631: F, t2014: F, t2021: F, t534: F) -> (F, F, F, F, F, F, F) {
    let t4677 = t3355 * t1013;
    let t4680 = t2007 * t4441;
    let t4683 = t528 * t4466;
    let t4686 = t4441 * t120;
    let t4687 = t72 * t4686;
    let t4690 = t4441 * t126;
    let t4693 = t4466 * t126;
    let t4698 = -0.11705142615505742e0 * t4680 * t120 + 0.23410285231011484e0 * t4683 * t120 - 0.26564305359272358183e-2 * t2014 * t4687 + 0.319782988780431561e-1 * t2021 * t4690 - 0.532971647967385935e-1 * t534 * t4693 + 0.13977476158628290272e-1 * t1631 * t4690;
    (t4677, t4680, t4683, t4687, t4690, t4693, t4698)
}
