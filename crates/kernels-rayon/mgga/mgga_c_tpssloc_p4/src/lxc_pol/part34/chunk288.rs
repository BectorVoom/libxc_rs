//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 288/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk288(t535: f64, t792: f64, t795: f64, t541: f64, t801: f64, t544: f64, t68: f64) -> (f64, f64, f64) {
    let t1322 = 0.41666666666666666666e-3_f64 * t792 * t535 * t795;
    let t1327 = 7.0_f64 / 288.0_f64 * t801 * t541;
    let t1336 = t544 * t68;
    (t1322, t1327, t1336)
}
