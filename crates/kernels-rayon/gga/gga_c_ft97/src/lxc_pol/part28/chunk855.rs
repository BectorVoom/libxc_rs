//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 855/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk855(t1852: f64, t34632: f64, t452: f64, t34563: f64, t83: f64, t34566: f64, t3238: f64, t7229: f64, t1332: f64, t6454: f64, t488: f64, t1339: f64, t1871: f64, t6469: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34634 = t452 * t1852 * t34632;
    let t34637 = t83 * t34563;
    let t34640 = t83 * t34566;
    let t34644 = t452 * t3238 * t7229;
    let t34647 = t6454 * t1332;
    let t34649 = t452 * t488 * t34647;
    let t34653 = t1871 * t1339 * t6469;
    (t34634, t34637, t34640, t34644, t34647, t34649, t34653)
}
