//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 977/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk977<F: Float>(t10108: F, t829: F, t160: F, t239: F, t330: F, t822: F, t1057: F, t2466: F, t1065: F, t2471: F, t323: F, t325: F, t8291: F) -> (F, F, F, F, F, F, F, F) {
    let t10109 = t10108 * t829;
    let t10112 = t160 * t239;
    let t10113 = F::cast_from(0.71734315950379065738e-1_f64) * t10112;
    let t10114 = t822 * t330;
    let t10115 = F::cast_from(0.62154466893555682512e-3_f64) * t10114;
    let t10131 = t2466 * t1057;
    let t10133 = t2471 * t1065;
    let t10137 = F::cast_from(0.77488888888888888888e-2_f64) * t323 * t8291 * t325;
    (t10109, t10112, t10113, t10114, t10115, t10131, t10133, t10137)
}
