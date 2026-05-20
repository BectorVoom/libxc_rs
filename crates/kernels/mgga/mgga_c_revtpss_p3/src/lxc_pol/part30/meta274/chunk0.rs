//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1212/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1212<F: Float>(t670: F, t7226: F, t7228: F, t7230: F, t7584: F, t7586: F, t118: F, t1310: F, t1453: F, t2127: F, t2163: F, t2165: F, t508: F, t569: F, t649: F, t651: F, t671: F, t6990: F, t6992: F, t6995: F, t7005: F, t7236: F, t7241: F, t7314: F, t7317: F, t7591: F, t7683: F) -> (F, F) {
    let t7687 = F::new(2.0) * t670 * t7586 + t7226 + t7228 + t7230 + t7584;
    let t7690 = -t118 * t7683 - t1310 * t2127 + t1453 * t2165 - t2163 * t649 - t508 * t7584 + t569 * t7687 - F::new(2.0) * t651 * t7591 - F::new(2.0) * t671 * t7586 - t6990 - t6992 - t6995 - t7005 + t7236 + t7241 + t7314 - t7317;
    (t7687, t7690)
}
