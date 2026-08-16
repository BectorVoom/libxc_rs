//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 918/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk918(t2013: f64, t3296: f64, t969: f64, t9829: f64, t825: f64, t2465: f64, t2571: f64, t2464: f64, t2194: f64, t3308: f64, t7068: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9845 = t2013 * t3296;
    let t9847 = t969 * t9829;
    let t9848 = t825 * t9847;
    let t9850 = t2465 * t2571;
    let t9851 = t2464 * t9850;
    let t9852 = t825 * t9851;
    let t9873 = t2194 * t3308;
    let t9889 = t883 * t7068;
    (t9845, t9847, t9848, t9850, t9851, t9852, t9873, t9889)
}
