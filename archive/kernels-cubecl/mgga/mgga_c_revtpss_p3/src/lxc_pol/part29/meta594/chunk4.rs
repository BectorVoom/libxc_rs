//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1993/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1993<F: Float>(t198: F, t7443: F, t2411: F, t28455: F, t1940: F, t2071: F, t580: F, t205: F, t7427: F, t1468: F, t2403: F, t25198: F, t25449: F, t26425: F, t26581: F, t27160: F, t27166: F, t27385: F, t27395: F, t28291: F, t28456: F, t28460: F, t28472: F, t4541: F, t605: F, t7092: F, t7428: F, t8020: F, t95511: F, t98688: F, t98733: F, t98760: F, t98787: F) -> (F, F, F, F, F) {
    let t102851 = t198 * t7443;
    let t102854 = t28455 * t2411;
    let t102858 = t1940 * t2071 * t580;
    let t102864 = t198 * t205 * t7427;
    let t102867 = -F::cast_from(3.0_f64) * t28291 * t98760 + F::cast_from(3.0_f64) * t26425 * t98688 - F::cast_from(3.0_f64) * t28472 * t98787 + F::cast_from(3.0_f64) * t2403 * t7428 * t27395 + F::cast_from(3.0_f64) * t4541 * t8020 * t25198 + t1940 * t26581 * t1468 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t26425 * t98733 - t1940 * t28460 * t25449 + F::cast_from(2.0_f64) * t102851 * t27385 - t1940 * t102854 * t7092 + t102858 + t1940 * t28456 * t605 - F::cast_from(3.0_f64) * t95511 * t27166 + F::cast_from(6.0_f64) * t102864 * t27160;
    (t102851, t102854, t102858, t102864, t102867)
}
