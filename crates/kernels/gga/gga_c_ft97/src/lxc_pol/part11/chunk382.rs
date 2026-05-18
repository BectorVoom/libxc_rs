//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 382/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk382<F: Float>(t126: F, t1595: F, t1631: F, t1655: F, t2009: F, t2012: F, t2014: F, t2016: F, t2021: F, t534: F) -> (F, F) {
    let t2022 = t1595 * t126;
    let t2030 = -F::new(0.11705142615505742e0) * t2009 + F::new(0.23410285231011484e0) * t2012 - F::new(0.26564305359272358183e-2) * t2014 * t2016 + F::new(0.319782988780431561e-1) * t2021 * t2022 - F::new(0.532971647967385935e-1) * t534 * t1655 * t126 + F::new(0.13977476158628290272e-1) * t1631 * t2022;
    (t2022, t2030)
}
