//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1095/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1095<F: Float>(t2311: F, t644: F, t77: F, t2315: F, t640: F, t10410: F, t84: F, t1925: F, t2247: F, t2251: F, t606: F, t2258: F, t10327: F, t603: F, t1928: F, t25106: F, t25114: F, t25120: F, t25140: F, t25143: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F) -> (F,) {
    let t92654 = t77 * t2311 * t644;
    let t92658 = t77 * t640 * t2315;
    let t92662 = t77 * t84 * t10410;
    let t92666 = t2247 * t2251 * t1925;
    let t92669 = t606 * t1925;
    let t92672 = t77 * t84 * t2258;
    let t92674 = t603 * t10327;
    let t92682 = 5.0 / 2.0 * t25106 * t25114 + 5.0 / 2.0 * t6958 * t92654 + 5.0 / 2.0 * t6958 * t92658 + 5.0 / 6.0 * t6958 * t92662 - 5.0 * t92666 * t6960 + t603 * t92669 * t92672 + t92674 * t1928 / 3.0 + t25120 * t6974 + t25120 * t6978 + t6963 * t25140 + 2.0 * t6963 * t25143;
    (t92682,)
}
