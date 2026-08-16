//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1290/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1290(t2221: f64, t3729: f64, t11661: f64, t23609: f64, t23612: f64, t829: f64, t11640: f64, t3235: f64, t11662: f64, t22866: f64, t23624: f64, t35813: f64, t6181: f64) -> (f64, f64, f64, f64, f64) {
    let t35903 = t2221 * t3729;
    let t35907 = t11661 * t23609 * t829 * t23612;
    let t35909 = t3235 * t11640;
    let t35912 = t11662 * t829 * t22866;
    let t35915 = t35813 * t6181 * t23624;
    (t35903, t35907, t35909, t35912, t35915)
}
