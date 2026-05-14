//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1007/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1007<F: Float>(t1927: F, t644: F, t1926: F, t1923: F, t1928: F, t25099: F, t25102: F, t25106: F, t25110: F, t25114: F, t25117: F, t25120: F, t25140: F, t25143: F, t25147: F, t25150: F, t25157: F, t25159: F, t25162: F, t6954: F, t6958: F, t6960: F, t6963: F, t6974: F, t6978: F) -> (F, F, F) {
    let t25163 = t1927 * t644;
    let t25164 = t1926 * t25163;
    let t25167 = 5.0 / 3.0 * t25099 * t6960 + 2.0 / 3.0 * t25102 * t1928 + 5.0 / 3.0 * t25106 * t6960 + 5.0 / 3.0 * t6958 * t25110 + 5.0 / 6.0 * t6958 * t25114 + t25117 * t1928 / 3.0 + t25120 * t1928 / 3.0 + 2.0 / 3.0 * t6963 * t6974 + 2.0 / 3.0 * t6963 * t6978 - t1923 * t25140 / 6.0 - t1923 * t25143 / 3.0 - t1923 * t25147 / 6.0 - t25150 * t1928 / 6.0 - t6954 * t6974 / 3.0 - t6954 * t6978 / 3.0 - 5.0 * t25157 * t25159 - 10.0 / 3.0 * t25162 * t25164;
    (t25163, t25164, t25167)
}
