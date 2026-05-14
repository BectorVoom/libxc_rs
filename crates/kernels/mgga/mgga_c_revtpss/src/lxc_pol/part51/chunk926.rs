//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 926/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk926<F: Float>(t25304: F, t31798: F, t119974: F, t1035: F, t1052: F, t8514: F, t8515: F, t3058: F, t31991: F, t8520: F, t25610: F, t126: F, t828: F, t32014: F, t32017: F, t31948: F, t94014: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120157 = t25304 * t31798;
    let t120159 = 0.50779446784275991476e-2 * t120157 * t119974;
    let t120179 = t1035 * t1052;
    let t120181 = t8514 * t8515 * t120179;
    let t120184 = t3058 * t31991;
    let t120190 = t8520 * t120179;
    let t120191 = t25610 * t120190;
    let t120199 = t828 * t126;
    let t120201 = t32014 * t120199 * t32017;
    let t120208 = t8514 * t94014 * t31948;
    (t120159, t120179, t120181, t120184, t120190, t120191, t120199, t120201, t120208)
}
