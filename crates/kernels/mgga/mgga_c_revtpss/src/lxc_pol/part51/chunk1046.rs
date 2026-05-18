//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1046/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1046<F: Float>(t31991: F, t93962: F, t1071: F, t31902: F, t1039: F, t246: F, t11874: F, t373: F, t385: F, t372: F, t3316: F, t7150: F) -> (F, F, F, F, F, F, F) {
    let t120297 = t93962 * t31991;
    let t120301 = t31902 * t1071 * t31991;
    let t120304 = t1039 * t246;
    let t120305 = t11874 * t120304;
    let t120306 = t373 * t385;
    let t120307 = t372 * t120306;
    let t120313 = t7150 * t3316 * t120304;
    (t120297, t120301, t120304, t120305, t120306, t120307, t120313)
}
