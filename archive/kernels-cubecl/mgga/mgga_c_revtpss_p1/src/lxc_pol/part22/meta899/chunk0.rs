//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3091/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3091<F: Float>(t15711: F, t3106: F, t15935: F, t372: F, t15904: F, t245: F, t3088: F, t12167: F, t1063: F, t1592: F, t247: F, t42778: F) -> (F, F, F, F, F, F) {
    let t53724 = t3106 * t15711;
    let t53728 = t372 * t15935;
    let t53739 = t15904 * t245;
    let t53740 = t3088 * t53739;
    let t53741 = t12167 * t53740;
    let t53762 = t1063 * t247 * t42778 * t1592;
    (t53724, t53728, t53739, t53740, t53741, t53762)
}
