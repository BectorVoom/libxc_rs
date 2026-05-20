//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 595/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk595<F: Float>(t3766: F, t460: F, t3601: F, t487: F, t3303: F, t3603: F, t1248: F, t1269: F, t1287: F, t3588: F, t1243: F, t3140: F) -> (F, F, F, F, F, F, F) {
    let t3767 = t460 * t3766;
    let t3768 = t487 * t3601;
    let t3769 = t3303 * t3603;
    let t3770 = t3768 * t3769;
    let t3774 = t1269 * t1248 * t1287;
    let t3778 = t487 * t3588 * t1287;
    let t3781 = t3140 * t1243;
    (t3767, t3768, t3769, t3770, t3774, t3778, t3781)
}
