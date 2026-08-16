//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1017/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1017(t3025: f64, t9972: f64, t8634: f64, t955: f64, t3464: f64, t773: f64, t1: f64, t3431: f64, t106: f64, t316: f64, t2089: f64, t723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10993 = 0.10725146985555128001e1_f64 * t3025 * t9972;
    let t10995 = 0.35750489951850426669e0_f64 * t955 * t8634;
    let t10996 = t773 * t3464;
    let t10999 = t3431 * t1;
    let t11000 = t10999 * t106;
    let t11001 = t11000 * t316;
    let t11004 = t2089 * t3431;
    let t11005 = t11004 * t723;
    (t10993, t10995, t10996, t10999, t11000, t11001, t11004, t11005)
}
