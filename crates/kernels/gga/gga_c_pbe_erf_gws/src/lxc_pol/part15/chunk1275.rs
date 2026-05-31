//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1275/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1275<F: Float>(t2408: F, t3060: F, t50881: F, t51084: F, t51572: F, t51595: F, t53700: F, t53704: F, t53713: F, t53715: F, t53721: F, t53726: F, t53728: F, t53730: F, t53734: F, t53736: F, t53742: F, t8629: F, t9283: F) -> F {
    let t53744 = -t53700 / F::cast_from(96.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t51572 - t53704 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51595 - t2408 * t9283 * t51084 * t3060 / F::cast_from(12.0_f64) - t53713 / F::cast_from(512.0_f64) + t53715 / F::cast_from(96.0_f64) - t53721 / F::cast_from(1536.0_f64) - t53726 + t53728 - t53730 + t53734 / F::cast_from(48.0_f64) - t53736 / F::cast_from(48.0_f64) + t8629 * t50881 / F::cast_from(96.0_f64) + t53742 / F::cast_from(1536.0_f64);
    t53744
}
