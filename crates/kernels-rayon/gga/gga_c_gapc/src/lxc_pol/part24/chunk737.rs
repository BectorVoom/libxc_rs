//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 737/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk737(t185: f64, t8769: f64, t1717: f64, t19: f64, t632: f64, t3114: f64, t1037: f64, t2999: f64, t520: f64, t1689: f64, t3006: f64, t3115: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8770 = t185 * t8769;
    let t8771 = t8770 * t1717;
    let t8774 = t632 * t19;
    let t8775 = t8774 * t3114;
    let t8776 = t1037 * t2999;
    let t8777 = t520 * t8776;
    let t8778 = t8775 * t8777;
    let t8780 = t1689 * t3006;
    let t8781 = t520 * t8780;
    let t8782 = t3115 * t8781;
    (t8770, t8771, t8775, t8776, t8778, t8780, t8782)
}
