//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1052/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1052(t140744: f64, t150042: f64, t33292: f64, t140762: f64, t140763: f64, t150071: f64, t35344: f64, t6109: f64, t681: f64, t193: f64, t3938: f64, t743: f64, t7484: f64) -> (f64, f64, f64, f64) {
    let t151030 = t33292 * t140744 * t150042;
    let t151033 = t140762 * t140763 * t150071;
    let t151035 = t6109 * t681 * t35344;
    let t151040 = t6109 * t193 * t743 * t7484 * t3938;
    (t151030, t151033, t151035, t151040)
}
