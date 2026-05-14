//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 831/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk831<F: Float>(t385: F, t988: F, t247: F, t3116: F, t1032: F, t7150: F, t8501: F) -> (F, F, F, F) {
    let t31914 = t385 * t988;
    let t31916 = t247 * t3116 * t31914;
    let t31919 = t7150 * t1032;
    let t31920 = t31919 * t8501;
    (t31914, t31916, t31919, t31920)
}
