//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 795/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk795<F: Float>(t599: F, t8901: F, t1181: F, t7337: F, t372: F, t525: F) -> (F, F, F, F) {
    let t8902 = t599 * t8901;
    let t8903 = t1181 * t8902;
    let t8904 = t7337 * t8903;
    let t8906 = t525 * t372;
    (t8902, t8903, t8904, t8906)
}
