//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 660/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk660<F: Float>(t236: F, t9145: F, t1971: F, t7453: F, t209: F, t476: F, t551: F, t3352: F, t1970: F, t558: F, t511: F, t570: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9146 = t236 * t9145;
    let t9147 = t1971 * t9146;
    let t9148 = t7453 * t9147;
    let t9151 = t551 * t476 * t209;
    let t9152 = t236 * t9151;
    let t9153 = t3352 * t9152;
    let t9154 = t1970 * t9153;
    let t9157 = t558 * t476 * t209;
    let t9158 = t511 * t9157;
    let t9159 = t1971 * t9158;
    let t9160 = t1970 * t9159;
    let t9163 = t570 * t476 * t209;
    (t9147, t9148, t9151, t9153, t9154, t9157, t9159, t9160, t9163)
}
