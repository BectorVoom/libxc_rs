//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1132;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta276<F: Float>(t283: F, t2857: F, t66: F, t11145: F, t247: F, t3298: F, t994: F, t4891: F, t3154: F, t999: F, t11659: F, t3117: F) -> (F, F, F, F, F, F, F, F) {
        let (t11852, t11853, t11855, t11858, t11859, t11860, t11861, t11862) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1132::<F>(t283, t2857, t66, t11145, t247, t3298, t994, t4891, t3154, t999, t11659, t3117);
    (t11852, t11853, t11855, t11858, t11859, t11860, t11861, t11862)
}
