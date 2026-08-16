//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1354;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta373(t2760: f64, t2783: f64, t786: f64, t2801: f64, t10069: f64, t10920: f64, t231: f64, t2782: f64, t39709: f64, t10910: f64, t233: f64, t689: f64, t869: f64, t2778: f64, t39515: f64, t39501: f64, t871: f64, t10115: f64, t225: f64, t880: f64, t10866: f64, t232: f64, t235: f64, t239: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40298, t40303, t40307, t40311) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1354(t2760, t2783, t786, t2801, t10069, t10920, t231, t2782, t39709, t10910, t233, t689, t869);
        let (t40314, t40316, t40317, t40318, t40321, t40324) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1355(t2778, t39515, t39501, t871, t10115, t225, t880, t10866, t232, t235, t239, t820);
    (t40298, t40303, t40307, t40311, t40314, t40316, t40317, t40318, t40321, t40324)
}
