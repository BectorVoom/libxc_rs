//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1007/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1007<F: Float>(t31002: F, t31016: F, t31021: F, t31023: F, t35176: F, t35184: F, t35186: F, t35191: F, t35195: F, t35199: F, t37426: F, t37435: F, t39746: F, t39750: F, t39756: F, t39763: F, t39765: F, t39767: F) -> (F,) {
    let t39769 = t31002 + t31016 - t31021 + t31023 + 0.10718504529517434243e-3 * t39746 + 0.10718504529517434243e-3 * t39750 - 0.41930789719472202756e-3 * t35176 + 0.53592522647587171215e-3 * t39756 + t37426 - 0.41930789719472202757e-3 * t35184 - 0.6431102717710460546e-2 * t35186 + t35191 - t35195 + t35199 - 0.94344276868812456204e-2 * t39763 - 0.42874018118069736972e-3 * t39765 - 0.28303283060643736861e-2 * t39767 - t37435;
    (t39769,)
}
