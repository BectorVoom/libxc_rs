//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3099/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3099<F: Float>(t16166: F, t3127: F, t3172: F, t16171: F, t42793: F, t4899: F, t4901: F, t11710: F, t16095: F, t16097: F, t16127: F, t43131: F) -> (F, F, F, F, F) {
    let t54042 = t3127 * t3172 * t16166;
    let t54047 = t3127 * t3172 * t16171;
    let t54078 = t4899 * t42793 * t4901;
    let t54081 = t16095 * t11710 * t16097;
    let t54085 = t16095 * t43131 * t16127;
    (t54042, t54047, t54078, t54081, t54085)
}
