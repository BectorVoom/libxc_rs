//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1100/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1100<F: Float>(t26992: F, t3500: F, t7788: F, t1014: F, t26828: F, t26851: F, t26972: F, t7768: F, t1141: F, t26866: F, t14443: F, t26702: F, t26685: F, t7703: F, t330: F, t9985: F) -> (F, F, F, F, F, F, F, F) {
    let t93196 = t7788 * t3500 * t26992;
    let t93211 = t1014 * t26828;
    let t93216 = t1014 * t26851;
    let t93222 = t7768 * t26972;
    let t93243 = t26866 * t1141;
    let t93341 = t14443 * t26702;
    let t93342 = t26685 * t93341;
    let t93344 = t7703 * t93341;
    let t93346 = t9985 * t330;
    (t93196, t93211, t93216, t93222, t93243, t93342, t93344, t93346)
}
