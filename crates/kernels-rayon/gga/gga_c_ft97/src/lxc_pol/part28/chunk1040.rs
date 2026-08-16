//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1040/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1040(t136433: f64, t3076: f64, t1554: f64, t1570: f64, t3188: f64, t32241: f64, t1557: f64, t7760: f64, t358: f64, t137037: f64, t3033: f64, t136815: f64, t1630: f64, t3037: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t145160 = t3076 * t136433;
    let t145163 = t32241 * t1554 * t1570 * t3188;
    let t145168 = t32241 * t7760 * t1557 * t3188;
    let t145171 = t1554 * t358;
    let t145188 = t137037 * t3033;
    let t145192 = t136815 * t1630 * t3037;
    (t145160, t145163, t145168, t145171, t145188, t145192)
}
