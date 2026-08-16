//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1001/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1001(t33293: f64, t33294: f64, t3628: f64, t3746: f64, t6108: f64, t27805: f64, t33319: f64, t9770: f64, t33476: f64, t3875: f64, t446: f64, t1434: f64, t35339: f64, t681: f64) -> (f64, f64, f64, f64, f64) {
    let t150114 = t6108 * t3628 * t33293 * t33294 * t3746;
    let t150118 = t27805 * t9770 * t33319 * t3746;
    let t150120 = t33476 * t3875;
    let t150122 = t446 * t9770 * t150120;
    let t150125 = t1434 * t681 * t35339;
    (t150114, t150118, t150120, t150122, t150125)
}
