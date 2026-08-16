//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1412/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1412(t12042: f64, t12591: f64, t12586: f64, t12655: f64, t12622: f64, t1616: f64, t687: f64, t35397: f64, t36332: f64, t36333: f64, t36334: f64, t36335: f64, t36336: f64, t36337: f64, t36338: f64, t36340: f64, t36341: f64, t36342: f64, t36343: f64, t36344: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37330 = 2.0_f64 * t12042;
    let t38531 = 4.0_f64 * t12591;
    let t38532 = 2.0_f64 * t12586;
    let t38534 = 2.0_f64 * t12655;
    let t38537 = 4.0_f64 * t1616 * t12622 * t687;
    let t38539 = t36332 - t36333 - t36334 - t36335 - t36336 - t36337 - t36338 + 0.53949325746737929041e-3_f64 * t35397 - t36340 - t36341 + t36342 - t36343 - t36344;
    (t37330, t38531, t38532, t38534, t38537, t38539)
}
