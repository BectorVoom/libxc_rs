//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2028/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2028<F: Float>(t26948: F, t487: F, t8945: F, t26936: F, t3736: F, t7635: F, t3566: F, t1269: F, t7642: F, t13032: F, t26848: F, t12881: F, t7624: F) -> (F, F, F, F, F, F, F) {
    let t97040 = t26948 * t487;
    let t97041 = t97040 * t8945;
    let t97050 = t26948 * t26936;
    let t97065 = t7635 * t3736;
    let t97066 = t3566 * t97065;
    let t97081 = t7642 * t1269;
    let t97082 = t97081 * t8945;
    let t97129 = t13032 * t26848;
    let t97141 = t7624 * t12881;
    (t97041, t97050, t97065, t97066, t97082, t97129, t97141)
}
