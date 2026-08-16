//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta240<F: Float>(t2453: F, t861: F, t2458: F, t2761: F, t786: F, t789: F, t212: F, t2760: F, t780: F, t689: F, t785: F, t860: F) -> (F, F, F, F, F, F, F, F) {
        let (t11018, t11019, t11021, t11022, t11024, t11025, t11026, t11028) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1044::<F>(t2453, t861, t2458, t2761, t786, t789, t212, t2760, t780, t689, t785, t860);
    (t11018, t11019, t11021, t11022, t11024, t11025, t11026, t11028)
}
