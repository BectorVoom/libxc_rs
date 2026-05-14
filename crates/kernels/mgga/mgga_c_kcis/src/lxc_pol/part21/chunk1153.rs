//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1153/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1153<F: Float>(t14375: F, t303: F, t7726: F, t13124: F, t26806: F, t2894: F, t4580: F, t7703: F, t922: F, t93425: F, t93569: F, t93592: F, t95670: F, t95865: F, t96105: F, t96108: F, t96116: F, t96121: F, t96124: F, t96127: F) -> (F, F, F) {
    let t96130 = t303 * t7726 * t14375;
    let t96133 = t303 * t7726 * t13124;
    let t96135 = 0.46336805555555555556e-3 * t7703 * t2894 * t95670 * t922 - 0.46336805555555555556e-3 * t93592 * t96105 - 0.92673611111111111112e-3 * t93592 * t96108 * t4580 * t26806 - 0.6183646701388888889e-4 * t93425 * t96105 - 0.22109259259259259258e-2 * t96116 - 0.8237654320987654321e-3 * t93569 + 0.13901041666666666667e-2 * t7703 * t95865 - 0.3684876543209876543e-3 * t96121 - t96124 + 0.49745833333333333332e-2 * t96127 - 0.24872916666666666666e-2 * t96130 - 0.24320185185185185185e-1 * t96133;
    (t96130, t96133, t96135)
}
