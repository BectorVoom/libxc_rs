//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 955/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk955<F: Float>(t4237: F, t84: F, t8621: F, t32135: F, t60224: F, t13272: F, t32148: F, t1470: F, t644: F, t8442: F, t6972: F, t640: F, t119457: F, t36: F, t606: F, t7714: F) -> (F, F, F, F, F, F, F, F) {
    let t125248 = t8621 * t84 * t4237;
    let t125251 = t60224 * t32135;
    let t125254 = t13272 * t32148;
    let t125257 = t13272 * t32135;
    let t125260 = t1470 * t644;
    let t125261 = t8442 * t125260;
    let t125265 = t8442 * t1470 * t6972;
    let t125268 = t1470 * t640;
    let t125269 = t119457 * t125268;
    let t125274 = t8442 * t7714 * t36 * t606;
    (t125248, t125251, t125254, t125257, t125261, t125265, t125269, t125274)
}
