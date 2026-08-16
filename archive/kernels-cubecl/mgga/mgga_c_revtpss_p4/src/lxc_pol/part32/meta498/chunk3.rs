//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1780/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1780<F: Float>(t25904: F, t28894: F, t25899: F, t213: F, t8085: F, t1904: F, t7492: F, t689: F, t225: F, t28888: F, t27899: F, t7515: F) -> (F, F, F, F, F, F, F) {
    let t28895 = t25904 * t28894;
    let t28897 = t25899 * t28894;
    let t28899 = t213 * t8085;
    let t28902 = t7492 * t1904;
    let t28903 = t689 * t28902;
    let t28905 = t28888 * t225;
    let t28909 = t27899 * t7515;
    (t28895, t28897, t28899, t28902, t28903, t28905, t28909)
}
