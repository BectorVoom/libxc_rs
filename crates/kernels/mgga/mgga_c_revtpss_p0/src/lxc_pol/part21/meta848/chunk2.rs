//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3188/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3188<F: Float>(t1196: F, t12494: F, t16642: F, t12552: F, t16811: F, t5180: F, t300: F, t3521: F, t1757: F, t58666: F, t12596: F, t5192: F) -> (F, F, F, F) {
    let t58703 = F::cast_from(0.14035736694323150897e2_f64) * t1196 * t16642 * t12494;
    let t58707 = F::cast_from(0.30762056574649219973e4_f64) * t1196 * t12552 * t5180 * t16811;
    let t58708 = t300 * t3521;
    let t58711 = F::cast_from(0.10526802520742363173e2_f64) * t58708 * t1757 * t58666;
    let t58713 = F::cast_from(0.35089341735807877242e1_f64) * t5192 * t12596;
    (t58703, t58707, t58711, t58713)
}
