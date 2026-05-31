//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 194/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk194<F: Float>(t293: F, t291: F, t135: F, t286: F, t455: F, t458: F, t456: F, t708: F, t295: F, t471: F, t64: F) -> (F, F, F, F, F, F, F) {
    let t711 = t293 * t293;
    let t712 = F::cast_from(1.0_f64) / t711;
    let t713 = t291 * t712;
    let t714 = t713 * t135;
    let t716 = t455 * t286 * t458;
    let t719 = -F::cast_from(7.0_f64) / F::cast_from(128.0_f64) * t456 * t286 * t708 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t714 * t716;
    let t723 = t719 * t471 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t295 * t64;
    (t711, t712, t713, t714, t716, t719, t723)
}
