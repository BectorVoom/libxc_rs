//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3118/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3118<F: Float>(t15830: F, t3111: F, t11866: F, t16035: F, t16088: F, t342: F, t380: F, t16219: F, t3241: F, t12047: F, t53552: F, t15810: F, t3127: F, t3172: F) -> (F, F, F, F, F, F) {
    let t55002 = t15830 * t3111;
    let t55004 = t11866 * t16035;
    let t55011 = t342 * t380 * t16088;
    let t55033 = t3241 * t16219;
    let t55046 = t12047 * t53552;
    let t55058 = t3127 * t3172 * t15810;
    (t55002, t55004, t55011, t55033, t55046, t55058)
}
