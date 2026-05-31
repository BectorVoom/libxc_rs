//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2712/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2712<F: Float>(t4186: F, t4401: F, t606: F, t749: F, t39737: F, t39766: F, t10433: F, t4311: F, t10489: F, t2403: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t39760: F, t39764: F, t39770: F, t4546: F) -> (F, F, F, F, F) {
    let t49911 = t4401 * t749 * t4186 * t606;
    let t49912 = F::cast_from(72.0_f64) * t49911;
    let t49913 = F::cast_from(24.0_f64) * t39737;
    let t49918 = F::cast_from(12.0_f64) * t39766;
    let t49920 = F::cast_from(4.0_f64) * t4311 * t10433;
    let t49921 = F::cast_from(3.0_f64) * t10489 * t2403 * t4546 + t39741 + t39744 + t39747 + t39750 + t39756 + t39760 - t39764 + t39770 + t49918 + t49920;
    (t49912, t49913, t49918, t49920, t49921)
}
