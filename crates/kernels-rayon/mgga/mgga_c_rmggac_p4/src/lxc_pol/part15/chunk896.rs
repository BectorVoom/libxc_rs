//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 896/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk896(t1614: f64, t1970: f64, t1971: f64, t209: f64, t511: f64, t605: f64, t1494: f64, t558: f64, t10030: f64, t7255: f64, t1652: f64, t515: f64) -> (f64, f64, f64, f64) {
    let t45012 = t1970 * t1971 * t511 * t1614 * t605 * t209;
    let t45018 = t1970 * t1971 * t511 * t558 * t1494 * t209;
    let t45020 = t7255 * t10030;
    let t45026 = t1970 * t1971 * t515 * t1652 * t605 * t209;
    (t45012, t45018, t45020, t45026)
}
