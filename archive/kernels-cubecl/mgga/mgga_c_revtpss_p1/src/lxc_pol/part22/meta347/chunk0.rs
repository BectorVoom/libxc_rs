//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1834/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1834<F: Float>(t11710: F, t3096: F, t3091: F, t1020: F, t3105: F, t247: F, t2862: F, t3109: F, t1063: F, t126: F, t3181: F, t2853: F) -> (F, F, F, F, F, F, F) {
    let t11711 = t11710 * t3096;
    let t11712 = t3091 * t11711;
    let t11714 = t1020 * t3105;
    let t11722 = t247 * t3109 * t2862;
    let t11723 = t1063 * t11722;
    let t11725 = t126 * t3181;
    let t11727 = t247 * t11725 * t2853;
    (t11711, t11712, t11714, t11722, t11723, t11725, t11727)
}
