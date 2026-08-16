//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1025/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1025<F: Float>(t31846: F, t839: F, t846: F, t8486: F, t241: F, t853: F, t125: F, t246: F, t775: F, t30: F, t7086: F, t33: F) -> (F, F, F, F, F, F, F, F) {
    let t31847 = t31846 * t839;
    let t31849 = t8486 * t846;
    let t31851 = t241 * t853;
    let t31853 = t246 * t125 * t775;
    let t31854 = t31851 * t31853;
    let t31855 = t8486 * t31854;
    let t31873 = t30 * t7086;
    let t32080 = t33 * t7086;
    (t31847, t31849, t31851, t31853, t31854, t31855, t31873, t32080)
}
