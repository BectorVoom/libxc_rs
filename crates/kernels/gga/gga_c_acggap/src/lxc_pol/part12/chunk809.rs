//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 809/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk809<F: Float>(t1327: F, t142: F, t8888: F, t599: F, t8406: F, t1181: F, t7346: F, t301: F, t525: F, t7337: F, t372: F, t604: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8889 = t142 * t1327;
    let t8890 = t8888 * t8889;
    let t8896 = t599 * t8406;
    let t8897 = t1181 * t8896;
    let t8898 = t7346 * t8897;
    let t8901 = t525 * t301;
    let t8902 = t599 * t8901;
    let t8903 = t1181 * t8902;
    let t8904 = t7337 * t8903;
    let t8906 = t525 * t372;
    let t8907 = t604 * t8906;
    (t8889, t8890, t8896, t8897, t8898, t8901, t8902, t8903, t8904, t8906, t8907)
}
