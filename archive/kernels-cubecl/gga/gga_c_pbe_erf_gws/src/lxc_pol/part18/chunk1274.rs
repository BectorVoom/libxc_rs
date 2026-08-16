//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1274/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1274<F: Float>(t1076: F, t1123: F, t833: F, t837: F, t850: F, t14677: F, t2503: F, t11396: F, t13888: F, t2408: F, t52931: F, t53083: F, t53779: F, t56110: F, t56113: F, t56116: F, t56119: F, t56124: F, t56126: F, t56129: F, t56133: F, t56142: F, t6793: F, t8793: F, t9283: F) -> F {
    let t56147 = t850 * t1123 * t1076 * t837 * t833;
    let t56153 = t14677 * t2503;
    let t56155 = -t56110 / F::cast_from(48.0_f64) - t56113 / F::cast_from(48.0_f64) + t56116 / F::cast_from(48.0_f64) + t56119 / F::cast_from(16.0_f64) + t56124 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t56126 - t56129 / F::cast_from(768.0_f64) - t6793 * t56133 / F::cast_from(12.0_f64) - t8793 * t53083 / F::cast_from(12.0_f64) - t8793 * t53779 / F::cast_from(12.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t56142 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t56147 - t52931 - t2408 * t9283 * t13888 * t11396 / F::cast_from(24.0_f64) + t56153 / F::cast_from(48.0_f64);
    t56155
}
