//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2964/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2964<F: Float>(t1063: F, t11725: F, t23481: F, t247: F, t24031: F, t999: F, t23474: F, t3109: F, t23847: F, t3127: F, t3172: F, t22688: F) -> (F, F, F, F, F) {
    let t78550 = t1063 * t247 * t11725 * t23481;
    let t78554 = t24031 * t999;
    let t78561 = t1063 * t247 * t3109 * t23474;
    let t78564 = t3127 * t3172 * t23847;
    let t78570 = t22688 * t999;
    (t78550, t78554, t78561, t78564, t78570)
}
