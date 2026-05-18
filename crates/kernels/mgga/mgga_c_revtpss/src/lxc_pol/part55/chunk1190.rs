//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1190/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1190<F: Float>(t126163: F, t32469: F, t32474: F, t119767: F, t1544: F, t247: F, t257: F, t837: F, t120046: F, t33721: F, t8486: F, t119875: F, t33682: F) -> (F, F, F, F, F) {
    let t126164 = t32469 * t126163;
    let t126166 = t32474 * t126163;
    let t126182 = t119767 * t247 * t257 * t1544 * t837;
    let t126185 = t8486 * t120046 * t33721;
    let t126210 = t119875 * t33682;
    (t126164, t126166, t126182, t126185, t126210)
}
