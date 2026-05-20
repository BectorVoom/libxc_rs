//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1874/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1874<F: Float>(t2142: F, t3738: F, t26969: F, t3566: F, t26936: F, t7642: F) -> (F, F, F) {
    let t26970 = t2142 * t3738;
    let t26971 = t26969 * t26970;
    let t26976 = t3566 * t2142;
    let t26979 = t7642 * t26936;
    (t26971, t26976, t26979)
}
