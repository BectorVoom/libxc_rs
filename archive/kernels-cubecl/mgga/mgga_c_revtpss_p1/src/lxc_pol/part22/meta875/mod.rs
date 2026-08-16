//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta875 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3039;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta875<F: Float>(t14574: F, t2439: F, t2777: F, t40297: F, t4500: F, t10069: F, t14504: F, t4423: F, t860: F, t1558: F, t2760: F, t14557: F, t9303: F, t4519: F, t9292: F, t2798: F, t4499: F, t9288: F, t10542: F, t14520: F, t2783: F, t4469: F, t786: F, t2801: F, t10073: F, t14588: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51355, t51371, t51373, t51375, t51380, t51390) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3039::<F>(t14574, t2439, t2777, t40297, t4500, t10069, t14504, t4423, t860, t1558, t2760, t14557, t9303);
        let (t51403, t51408, t51418, t51421, t51422, t51424) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3040::<F>(t4519, t9292, t2798, t4499, t9288, t10542, t14520, t2783, t4469, t786, t2801, t10073, t14588);
    (t51355, t51371, t51373, t51375, t51380, t51390, t51403, t51408, t51418, t51421, t51422, t51424)
}
