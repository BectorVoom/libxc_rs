//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1035/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1035<F: Float>(t32591: F, t606: F, t8442: F, t1925: F, t84: F, t640: F, t8621: F, t7002: F, t93: F, t1419: F, t3140: F, t8477: F) -> (F, F, F, F, F, F) {
    let t32592 = t32591 * t606;
    let t32593 = t8442 * t32592;
    let t32600 = t84 * t1925;
    let t32602 = t8621 * t32600 * t640;
    let t32655 = t93 * t7002;
    let t32699 = t1419 * t3140;
    let t32700 = t8477 * t32699;
    (t32593, t32600, t32602, t32655, t32699, t32700)
}
