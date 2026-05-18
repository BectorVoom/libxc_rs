//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1325/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1325<F: Float>(t26686: F, t3040: F, t4781: F, t14382: F, t3200: F, t95911: F, t2173: F, t46978: F, t8041: F, t7690: F, t96305: F, t14654: F, t3489: F) -> (F, F, F, F, F) {
    let t96372 = t26686 * t4781 * t3040;
    let t96379 = t3200 * t95911 * t14382;
    let t96382 = t2173 * t46978 * t8041;
    let t96388 = t7690 * t96305;
    let t96391 = t14654 * t3489;
    (t96372, t96379, t96382, t96388, t96391)
}
