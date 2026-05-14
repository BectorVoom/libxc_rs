//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 910/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk910<F: Float>(t26409: F, t2161: F, t2770: F, t2153: F, t2626: F, t2538: F, t7630: F, t826: F, t7655: F, t898: F, t2165: F, t2772: F, t874: F, t9194: F, t2157: F, t710: F, t7603: F, t86: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26410 = 2.0 * t26409;
    let t26411 = t2161 * t2770;
    let t26416 = t2153 * t2626;
    let t26417 = t2538 * t26416;
    let t26418 = 2.0 * t26417;
    let t26419 = t7630 * t826;
    let t26420 = t2538 * t26419;
    let t26421 = 4.0 * t26420;
    let t26422 = t7655 * t898;
    let t26425 = t2165 * t2772;
    let t26430 = t874 * t9194;
    let t26431 = t26430 * t2157;
    let t26434 = t86 * t710 * t7603;
    (t26410, t26411, t26416, t26417, t26418, t26419, t26420, t26421, t26422, t26425, t26430, t26431, t26434)
}
