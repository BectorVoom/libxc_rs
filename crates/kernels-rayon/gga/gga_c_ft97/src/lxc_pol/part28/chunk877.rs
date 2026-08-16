//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 877/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk877(t28: f64, t34931: f64, t89: f64, t32979: f64, t920: f64, t1969: f64, t446: f64, t1017: f64, t32709: f64, t34918: f64, t526: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34932 = t28 * t34931;
    let t34933 = t89 * t34932;
    let t34935 = t32979 * t920;
    let t34936 = t1969 * t34935;
    let t34937 = t446 * t34936;
    let t34939 = t32709 * t1017;
    let t34940 = t28 * t34939;
    let t34941 = t89 * t34940;
    let t34943 = t526 * t34918;
    let t34945 = t89 * t27 * t34943;
    (t34933, t34936, t34937, t34939, t34941, t34943, t34945)
}
