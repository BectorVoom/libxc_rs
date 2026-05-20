//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2717/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2717<F: Float>(t21101: F, t3707: F, t17608: F, t5292: F, t17547: F, t5265: F, t1261: F, t20906: F, t3172: F, t17416: F, t5391: F, t21272: F, t3636: F) -> (F, F, F, F, F, F) {
    let t70082 = t3707 * t21101;
    let t70088 = t17608 * t5292;
    let t70091 = t17547 * t5265;
    let t70102 = t1261 * t3172 * t20906;
    let t70112 = t5391 * t17416;
    let t70114 = t21272 * t3636;
    (t70082, t70088, t70091, t70102, t70112, t70114)
}
