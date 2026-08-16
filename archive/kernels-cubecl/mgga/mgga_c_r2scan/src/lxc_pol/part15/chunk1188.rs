//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1188/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1188<F: Float>(t1048: F, t39383: F, t39418: F, t39450: F, t39478: F, t39504: F, t39537: F, t39567: F, t39590: F, t39624: F, t39652: F, t39681: F, t39710: F, t39735: F, t39754: F, t39784: F, t39809: F, t39837: F, t39871: F, t39896: F, t39932: F, t39956: F, t39989: F, t40023: F, t40055: F, t40088: F, t40111: F, t40144: F, t40167: F, t40193: F, t40230: F, t40247: F, t40262: F, t499: F, t797: F) -> F {
    let t40271 = t1048 * t499 * (t39710 + t40111 + t40144 + t40247 + t39681 + t39956 + t39754 + t40262 + t39989 + t40167 + t39837 + t39784 + t39871 + t39652 + t40230 + t40088 + t39896 + t39504 + t39383 + t39450 + t40023 + t39567 + t40055 + t39590 + t39478 + t40193 + t39624 + t39418 + t39932 + t39809 + t39537 + t39735) * t797 / F::cast_from(4.0_f64);
    t40271
}
