//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1354;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta373<F: Float>(t2760: F, t2783: F, t786: F, t2801: F, t10069: F, t10920: F, t231: F, t2782: F, t39709: F, t10910: F, t233: F, t689: F, t869: F, t2778: F, t39515: F, t39501: F, t871: F, t10115: F, t225: F, t880: F, t10866: F, t232: F, t235: F, t239: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40298, t40303, t40307, t40311) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1354::<F>(t2760, t2783, t786, t2801, t10069, t10920, t231, t2782, t39709, t10910, t233, t689, t869);
        let (t40314, t40316, t40317, t40318, t40321, t40324) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1355::<F>(t2778, t39515, t39501, t871, t10115, t225, t880, t10866, t232, t235, t239, t820);
    (t40298, t40303, t40307, t40311, t40314, t40316, t40317, t40318, t40321, t40324)
}
