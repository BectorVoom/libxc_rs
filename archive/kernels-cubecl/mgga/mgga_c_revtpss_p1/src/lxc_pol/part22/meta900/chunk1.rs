//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3094/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3094<F: Float>(t366: F, t53865: F, t1025: F, t371: F, t4852: F, t676: F, t225: F, t53014: F, t11656: F, t15734: F, t11670: F, t370: F) -> (F, F, F, F, F) {
    let t53866 = t53865 * t366;
    let t53875 = t1025 * t371 * t676 * t4852;
    let t53877 = t53014 * t225;
    let t53881 = t11656 * t15734;
    let t53884 = t11670 * t370;
    (t53866, t53875, t53877, t53881, t53884)
}
