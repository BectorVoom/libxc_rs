//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 458/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk458<F: Float>(t120: F, t4441: F, t72: F, t126: F, t4466: F, t1631: F, t2014: F, t2021: F, t4680: F, t4683: F, t534: F, t139: F) -> (F, F, F, F) {
    let t4686 = t4441 * t120;
    let t4687 = t72 * t4686;
    let t4690 = t4441 * t126;
    let t4693 = t4466 * t126;
    let t4698 = -F::new(0.11705142615505742e0) * t4680 * t120 + F::new(0.23410285231011484e0) * t4683 * t120 - F::new(0.26564305359272358183e-2) * t2014 * t4687 + F::new(0.319782988780431561e-1) * t2021 * t4690 - F::new(0.532971647967385935e-1) * t534 * t4693 + F::new(0.13977476158628290272e-1) * t1631 * t4690;
    let t4699 = t139 * t4698;
    (t4686, t4687, t4698, t4699)
}
