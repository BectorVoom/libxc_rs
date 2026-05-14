//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1025/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1025<F: Float>(t1867: F, t3570: F, t5946: F, t997: F, t14056: F, t6328: F, t12587: F, t12599: F, t12601: F, t12603: F, t12608: F, t15486: F, t15497: F, t15501: F, t15508: F, t1008: F) -> (F, F) {
    let t20453 = t3570 * t1867;
    let t20455 = t997 * t5946;
    let t20459 = t14056 * t6328;
    let t20467 = -0.20579528696673473746e-1 * t15486 + 35.0 / 108.0 * t20453 + 0.80031500487063509015e-1 * t20455 + 0.10289764348336736873e-1 * t15497 - 0.85748036236139473944e-3 * t12587 + 0.13719685797782315831e-1 * t20459 + 0.96037800584476210817e-1 * t15501 - 0.90702367218671976886e-1 * t12599 + 0.90702367218671976886e-1 * t12601 - 0.68026775414003982664e-1 * t12603 + 0.25724410870841842183e-2 * t15508 - 0.42874018118069736972e-2 * t12608;
    let t20471 = t1008 * t5946;
    (t20467, t20471)
}
