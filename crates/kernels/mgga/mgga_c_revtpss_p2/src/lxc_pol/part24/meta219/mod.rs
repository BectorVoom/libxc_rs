//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk967;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta219<F: Float>(t271: F, t2857: F, t11144: F, t11150: F, t3252: F, t283: F, t66: F, t3298: F, t994: F, t4891: F, t3316: F, t11132: F, t126: F, t373: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11821, t11822, t11827, t11852, t11853, t11858, t11859, t11874, t11875, t11890) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk967::<F>(t271, t2857, t11144, t11150, t3252, t283, t66, t3298, t994, t4891, t3316, t11132);
        let (t11921, t11922) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk968::<F>(t126, t373, t828);
    (t11821, t11822, t11827, t11852, t11853, t11858, t11859, t11874, t11875, t11890, t11921, t11922)
}
