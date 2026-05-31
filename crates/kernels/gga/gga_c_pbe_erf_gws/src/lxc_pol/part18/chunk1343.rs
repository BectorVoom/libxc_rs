//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1343/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1343<F: Float>(t15255: F, t51382: F, t3799: F, t4033: F, t3867: F, t51388: F, t51396: F, t54302: F, t55582: F, t57127: F, t57130: F, t57132: F, t57134: F, t57138: F, t57140: F) -> F {
    let t57142 = t51382 * t15255;
    let t57144 = t4033 * t3799;
    let t57146 = t4033 * t3867;
    let t57148 = -t57127 / F::cast_from(4.0_f64) + t57130 / F::cast_from(8.0_f64) + t57132 / F::cast_from(48.0_f64) - t57134 / F::cast_from(384.0_f64) - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t51388 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t51396 + t57138 / F::cast_from(24.0_f64) + t54302 - t55582 - t57140 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57142 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t57144 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57146;
    t57148
}
