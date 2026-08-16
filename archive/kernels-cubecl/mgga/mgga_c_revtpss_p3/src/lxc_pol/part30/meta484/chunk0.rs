//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1822/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1822<F: Float>(t2022: F, t4077: F, t25924: F, t4075: F, t7282: F, t1955: F) -> (F, F, F, F) {
    let t25925 = t2022 * t4077;
    let t25926 = t25924 * t25925;
    let t25929 = t7282 * t4075;
    let t25930 = t1955 * t25929;
    (t25925, t25926, t25929, t25930)
}
