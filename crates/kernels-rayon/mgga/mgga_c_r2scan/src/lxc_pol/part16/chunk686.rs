//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 686/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk686(t224: f64, t5290: f64, t1821: f64, t5270: f64, t234: f64, t1809: f64, t1841: f64, t720: f64, t1819: f64, t1814: f64, t732: f64, t1813: f64) -> (f64, f64, f64, f64, f64) {
    let t5291 = t5290 * t224;
    let t5292 = t1821 * t5270;
    let t5293 = t5291 * t5292;
    let t5295 = 0.12304822629859687989e5_f64 * t234 * t5293;
    let t5296 = t1841 * t1809;
    let t5298 = 0.10526802520742363173e2_f64 * t234 * t5296;
    let t5299 = t720 * t5270;
    let t5300 = t1819 * t5299;
    let t5302 = 0.6233709278045326953e3_f64 * t234 * t5300;
    let t5303 = t732 * t1814;
    let t5305 = t1841 * t1813;
    (t5295, t5298, t5302, t5303, t5305)
}
