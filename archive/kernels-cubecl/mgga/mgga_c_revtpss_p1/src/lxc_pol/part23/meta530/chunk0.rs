//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2053/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2053<F: Float>(t21464: F, t21516: F, t21568: F, t21615: F, t1277: F, t20849: F, t487: F) -> (F, F, F) {
    let t21617 = t21464 + t21516 + t21568 + t21615;
    let t21618 = t1277 * t21617;
    let t21621 = t20849 * t487;
    (t21617, t21618, t21621)
}
