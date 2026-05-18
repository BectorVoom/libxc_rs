//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1210/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1210<F: Float>(t4241: F, t8621: F, t8622: F, t1925: F, t640: F, t1493: F, t32600: F, t4237: F, t1921: F, t8766: F, t2167: F, t7956: F) -> (F, F, F, F, F) {
    let t128444 = t8621 * t8622 * t4241;
    let t128449 = t640 * t1925;
    let t128451 = t8621 * t128449 * t1493;
    let t128457 = t8621 * t32600 * t4237;
    let t129138 = t8766 * t1921;
    let t129141 = t2167 * t7956;
    (t128444, t128451, t128457, t129138, t129141)
}
