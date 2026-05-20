//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2957/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2957<F: Float>(t13396: F, t4786: F, t1086: F, t15654: F, t3090: F, t11922: F, t16077: F, t3115: F, t225: F, t53222: F, t366: F, t1025: F, t371: F, t4852: F, t676: F) -> (F, F, F, F, F, F) {
    let t53846 = t13396 * t4786;
    let t53855 = t15654 * t1086 * t3090;
    let t53859 = t3115 * t11922 * t16077;
    let t53865 = t53222 * t225;
    let t53866 = t53865 * t366;
    let t53875 = t1025 * t371 * t676 * t4852;
    (t53846, t53855, t53859, t53865, t53866, t53875)
}
