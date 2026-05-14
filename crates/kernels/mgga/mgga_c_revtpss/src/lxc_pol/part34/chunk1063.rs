//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1063/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1063<F: Float>(t30066: F, t30109: F, t532: F, t1450: F, t2014: F, t1518: F, t7883: F, t2007: F, t5920: F, t1868: F, t1907: F, t8717: F, t25082: F, t7732: F, t7742: F, t1936: F, t6765: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30110 = t30066 + t30109;
    let t30111 = t532 * t30110;
    let t30112 = t30111 * t1450;
    let t30113 = t2014 * t30112;
    let t30116 = t7883 * t1518;
    let t30119 = t2007 * t5920;
    let t30122 = t1868 * t1907;
    let t30123 = t8717 * t30122;
    let t30125 = 6.0 * t25082 * t30123;
    let t30127 = 4.0 * t7732 * t7742;
    let t30128 = t6765 * t1936;
    (t30110, t30111, t30112, t30113, t30116, t30119, t30122, t30123, t30125, t30127, t30128)
}
