//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1081/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1081<F: Float>(t4241: F, t8441: F, t8621: F, t28076: F, t1493: F, t640: F, t4237: F, t84: F, t32135: F, t60224: F, t13272: F, t32148: F) -> (F, F, F, F, F, F) {
    let t125228 = t8621 * t8441 * t4241;
    let t125238 = t8621 * t8441 * t28076;
    let t125244 = t8621 * t640 * t1493;
    let t125248 = t8621 * t84 * t4237;
    let t125251 = t60224 * t32135;
    let t125254 = t13272 * t32148;
    (t125228, t125238, t125244, t125248, t125251, t125254)
}
