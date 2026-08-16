//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1010/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1010(t11364: f64, t11473: f64, t300: f64, t11128: f64, t11133: f64, t11179: f64, t11182: f64, t11184: f64, t11187: f64, t11194: f64, t11272: f64, t11280: f64, t11288: f64, t11290: f64, t11296: f64) -> (f64, f64) {
    let t11475 = t300 * (t11364 + t11473);
    let t11476 = -t11128 - t11133 + t11179 + t11182 + t11184 + t11187 - t11194 + t11272 + t11280 - t11288 + t11290 + t11296 + t11475;
    (t11475, t11476)
}
