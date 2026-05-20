//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2325;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta600<F: Float>(t2452: F, t588: F, t258: F, t2454: F, t2455: F, t39494: F, t10985: F, t11018: F, t2439: F, t2760: F, t780: F, t785: F, t11028: F, t887: F, t11021: F, t2471: F, t11024: F, t689: F, t2440: F, t2772: F, t10541: F, t2453: F, t10538: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t39552, t39554, t39557, t39558, t39562) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2325::<F>(t2452, t588, t258, t2454, t2455, t39494, t10985, t11018, t2439, t2760, t780, t785);
        let (t39565, t39567, t39570, t39573, t39575, t39576) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2326::<F>(t11028, t2439, t887, t11021, t2471, t11024, t689, t2440, t2772, t10541, t2453, t10538);
    (t39552, t39554, t39557, t39558, t39562, t39565, t39567, t39570, t39573, t39575, t39576)
}
