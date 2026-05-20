//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2979/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2979<F: Float>(t40270: F, t5737: F, t13920: F, t555: F, t10073: F, t14207: F, t2782: F, t4086: F, t47973: F, t543: F, t10090: F, t13805: F, t1882: F, t2482: F, t686: F, t72: F) -> (F, F, F, F, F) {
    let t49210 = t40270 * t5737;
    let t49213 = t555 * t13920;
    let t49238 = t10073 * t14207;
    let t49242 = t2782 * t4086 * t47973 * t543;
    let t49248 = t2482 * t10090 * t1882 * t13805 * t72 * t686;
    (t49210, t49213, t49238, t49242, t49248)
}
