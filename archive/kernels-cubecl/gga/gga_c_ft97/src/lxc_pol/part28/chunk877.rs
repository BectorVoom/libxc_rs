//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 877/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk877<F: Float>(t28: F, t34931: F, t89: F, t32979: F, t920: F, t1969: F, t446: F, t1017: F, t32709: F, t34918: F, t526: F, t27: F) -> (F, F, F, F, F, F, F) {
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
