//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 966/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk966<F: Float>(t26265: F, t5722: F, t28845: F, t7289: F, t689: F, t8099: F, t25904: F, t25899: F, t213: F, t8085: F, t1904: F, t7492: F, t27899: F, t7515: F, t2097: F, t3999: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28853 = t26265 * t5722;
    let t28858 = t7289 * t28845;
    let t28894 = t8099 * t689;
    let t28895 = t25904 * t28894;
    let t28897 = t25899 * t28894;
    let t28899 = t213 * t8085;
    let t28902 = t7492 * t1904;
    let t28903 = t689 * t28902;
    let t28909 = t27899 * t7515;
    let t28911 = t3999 * t2097;
    (t28853, t28858, t28894, t28895, t28897, t28899, t28902, t28903, t28909, t28911)
}
