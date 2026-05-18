//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1146/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1146<F: Float>(t1972: F, t3204: F, t3143: F, t3148: F, t3141: F, t7120: F, t3123: F, t7121: F, t365: F, t3089: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25500 = t3204 * t1972;
    let t25503 = t3143 * sigma0;
    let t25504 = t25503 * t3148;
    let t25505 = t3141 * t25504;
    let t25508 = t7120 * t3148;
    let t25509 = t3141 * t25508;
    let t25512 = t3123 * t7121;
    let t25515 = sigma0 * t365;
    let t25516 = t25515 * t3089;
    (t25500, t25503, t25504, t25505, t25508, t25509, t25512, t25515, t25516)
}
