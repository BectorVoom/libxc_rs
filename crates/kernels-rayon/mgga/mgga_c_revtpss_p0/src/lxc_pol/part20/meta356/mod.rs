//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1298;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1299;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta356(t2452: f64, t588: f64, t258: f64, t2454: f64, t2455: f64, t39494: f64, t10985: f64, t11018: f64, t2439: f64, t2760: f64, t780: f64, t785: f64, t11028: f64, t887: f64, t11021: f64, t2471: f64, t11024: f64, t689: f64, t2440: f64, t2772: f64, t10541: f64, t2453: f64, t10538: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39552, t39554, t39557, t39558, t39562) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1298(t2452, t588, t258, t2454, t2455, t39494, t10985, t11018, t2439, t2760, t780, t785);
        let (t39565, t39567, t39570, t39573, t39576) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1299(t11028, t2439, t887, t11021, t2471, t11024, t689, t2440, t2772, t10541, t2453, t10538);
    (t39552, t39554, t39557, t39558, t39562, t39565, t39567, t39570, t39573, t39576)
}
