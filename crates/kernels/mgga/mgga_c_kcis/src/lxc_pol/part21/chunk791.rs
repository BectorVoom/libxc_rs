//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 791/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk791<F: Float>(t13173: F, t4546: F, t3210: F, t13172: F, t3183: F, t4999: F, t1092: F, t5168: F, sigma0: F) -> (F, F, F, F) {
    let t13174 = t4546 * t13173;
    let t13175 = t3210 * t13174;
    let t13176 = t13172 * t13175;
    let t13178 = t4999 * t3183;
    let t13179 = t1092 * t13178;
    let t13181 = t5168 * sigma0;
    (t13174, t13176, t13179, t13181)
}
