//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2843/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2843<F: Float>(t11986: F, t828: F, t3091: F, t3096: F, t12097: F, t3090: F, t11273: F, t12012: F, t11631: F, t3133: F, t1086: F, t11223: F) -> (F, F, F, F, F, F) {
    let t43240 = t828 * t11986;
    let t43242 = t3091 * t43240 * t3096;
    let t43244 = t12097 * t3090;
    let t43268 = t11273 * t12012;
    let t43279 = t11631 * t3133;
    let t43285 = t11223 * t1086 * t3090;
    (t43240, t43242, t43244, t43268, t43279, t43285)
}
