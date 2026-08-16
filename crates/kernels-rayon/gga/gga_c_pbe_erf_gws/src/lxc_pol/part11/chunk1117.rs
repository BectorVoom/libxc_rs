//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1117/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1117(t12556: f64, t2615: f64, t12634: f64, t5218: f64, t7495: f64, t12804: f64, t24848: f64, t18224: f64, t47809: f64, t47810: f64, t47811: f64, t47812: f64, t47814: f64, t47818: f64, t47820: f64) -> (f64, f64, f64, f64) {
    let t47822 = 128.0_f64 / 81.0_f64 * t2615 * t12556;
    let t47825 = 32.0_f64 / 15.0_f64 * t5218 * t7495 * t12634;
    let t47828 = 32.0_f64 / 9.0_f64 * t5218 * t24848 * t12804;
    let t47829 = t47809 + t47810 - t47811 - t47812 + t18224 + t47814 + t47818 + t47820 + t47822 - t47825 - t47828;
    (t47822, t47825, t47828, t47829)
}
