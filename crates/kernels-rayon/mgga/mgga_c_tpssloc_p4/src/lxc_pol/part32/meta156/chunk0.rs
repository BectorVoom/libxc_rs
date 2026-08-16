//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 820/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk820(t225: f64, t4210: f64, t4217: f64, t228: f64, t68: f64, t1484: f64, t845: f64, t776: f64, t4119: f64, t824: f64, t1504: f64, t1506: f64, t230: f64, t822: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4219 = (t4210 + t4217) * t225;
    let t4225 = t228 * t68;
    let t4226 = t845 * t1484;
    let t4227 = t4226 * t776;
    let t4230 = t824 * t4119;
    let t4233 = 3.0_f64 * t1504 * t825 + 3.0_f64 * t1506 * t822 + 3.0_f64 * t228 * t4230 - t230 * t4219 - 12.0_f64 * t4225 * t4227;
    (t4219, t4225, t4226, t4227, t4230, t4233)
}
