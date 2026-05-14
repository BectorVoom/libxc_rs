//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 948/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk948<F: Float>(t125016: F, t8938: F, t12915: F, t247: F, t33398: F, t33399: F, t124571: F, t7657: F, t33405: F, t33406: F, t33529: F, t3801: F, t12587: F, t8951: F, t44126: F, t8955: F) -> (F, F, F, F, F, F, F) {
    let t125017 = t8938 * t125016;
    let t125028 = t33398 * t247 * t12915 * t33399;
    let t125048 = t124571 * t7657;
    let t125059 = t33405 * t247 * t12915 * t33406;
    let t125070 = t33529 * t3801;
    let t125074 = t8951 * t12587;
    let t125092 = t8955 * t44126;
    (t125017, t125028, t125048, t125059, t125070, t125074, t125092)
}
