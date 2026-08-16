//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1993/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1993(t198: f64, t7443: f64, t2411: f64, t28455: f64, t1940: f64, t2071: f64, t580: f64, t205: f64, t7427: f64, t1468: f64, t2403: f64, t25198: f64, t25449: f64, t26425: f64, t26581: f64, t27160: f64, t27166: f64, t27385: f64, t27395: f64, t28291: f64, t28456: f64, t28460: f64, t28472: f64, t4541: f64, t605: f64, t7092: f64, t7428: f64, t8020: f64, t95511: f64, t98688: f64, t98733: f64, t98760: f64, t98787: f64) -> (f64, f64, f64, f64, f64) {
    let t102851 = t198 * t7443;
    let t102854 = t28455 * t2411;
    let t102858 = t1940 * t2071 * t580;
    let t102864 = t198 * t205 * t7427;
    let t102867 = -3.0_f64 * t28291 * t98760 + 3.0_f64 * t26425 * t98688 - 3.0_f64 * t28472 * t98787 + 3.0_f64 * t2403 * t7428 * t27395 + 3.0_f64 * t4541 * t8020 * t25198 + t1940 * t26581 * t1468 / 2.0_f64 - 3.0_f64 * t26425 * t98733 - t1940 * t28460 * t25449 + 2.0_f64 * t102851 * t27385 - t1940 * t102854 * t7092 + t102858 + t1940 * t28456 * t605 - 3.0_f64 * t95511 * t27166 + 6.0_f64 * t102864 * t27160;
    (t102851, t102854, t102858, t102864, t102867)
}
