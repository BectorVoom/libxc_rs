//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 630/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk630<F: Float>(t4581: F, t5310: F, t737: F, t992: F, t1253: F, t167: F, t1852: F, t25: F, t1251: F, t1851: F, t330: F, t829: F, t3515: F, t286: F, t287: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5311 = t5310 * t4581;
    let t5314 = t737 * t992;
    let t5315 = t1253 * t167;
    let t5316 = t5314 * t5315;
    let t5321 = t25 * t1852;
    let t5322 = t1251 * t5321;
    let t5324 = t1851 * t330;
    let t5325 = t5324 * t829;
    let t5326 = t3515 * t5325;
    let t5329 = t286 * t287;
    (t5311, t5314, t5315, t5316, t5321, t5322, t5324, t5325, t5326, t5329)
}
