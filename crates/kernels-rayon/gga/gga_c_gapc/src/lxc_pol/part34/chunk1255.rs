//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1255/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1255(t11601: f64, t9288: f64, t1030: f64, t33748: f64, t8853: f64, t9278: f64, t26698: f64, t33399: f64, t8362: f64, t8784: f64, t11302: f64, t19902: f64, t20596: f64) -> (f64, f64, f64, f64, f64) {
    let t34936 = t11601 * t9288;
    let t34940 = t1030 * t33748 * t8853;
    let t34942 = t11601 * t9278;
    let t34946 = t8784 * t33399 * t8362 * t26698;
    let t34949 = t19902 * t11302 * t20596;
    (t34936, t34940, t34942, t34946, t34949)
}
