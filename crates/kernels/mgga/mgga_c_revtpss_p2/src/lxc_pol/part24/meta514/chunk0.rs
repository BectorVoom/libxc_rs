//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1532/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1532<F: Float>(t1065: F, t23598: F, t11630: F, t23829: F, t3172: F, t1011: F, t140: F, t24016: F, t11710: F, t23907: F, t3091: F, t23912: F) -> (F, F, F, F, F) {
    let t79301 = t1065 * t23598;
    let t79309 = t11630 * t3172 * t23829;
    let t79315 = t1011 * t140 * t24016;
    let t79428 = t3091 * t11710 * t23907;
    let t79439 = t3091 * t11710 * t23912;
    (t79301, t79309, t79315, t79428, t79439)
}
