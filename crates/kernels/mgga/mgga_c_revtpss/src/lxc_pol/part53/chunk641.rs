//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 641/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk641<F: Float>(t2014: F, t7316: F, t118: F, t1310: F, t1453: F, t1932: F, t2007: F, t2011: F, t508: F, t569: F, t649: F, t651: F, t671: F, t6983: F, t6985: F, t6990: F, t6992: F, t6995: F, t7005: F, t7007: F, t7221: F, t7231: F, t7236: F, t7241: F, t7314: F) -> (F, F) {
    let t7317 = t2014 * t7316;
    let t7318 = -t118 * t7221 - t1310 * t1932 + t1453 * t2011 - t2007 * t649 - t508 * t6983 + t569 * t7231 - 2.0 * t651 * t7007 - 2.0 * t671 * t6985 - t6990 - t6992 - t6995 - t7005 + t7236 + t7241 + t7314 - t7317;
    (t7317, t7318)
}
