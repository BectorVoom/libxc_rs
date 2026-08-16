//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2398/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2398<F: Float>(t11852: F, t126: F, t12166: F, t15905: F, t994: F, t11631: F, t999: F, t3046: F, t3298: F, t4891: F, t1052: F, t11243: F) -> (F, F, F, F, F) {
    let t42534 = t126 * t11852;
    let t42621 = t994 * t12166 * t15905;
    let t42622 = t11631 * t999;
    let t42643 = t3046 * t3298 * t4891;
    let t42646 = t1052 * t11243;
    (t42534, t42621, t42622, t42643, t42646)
}
