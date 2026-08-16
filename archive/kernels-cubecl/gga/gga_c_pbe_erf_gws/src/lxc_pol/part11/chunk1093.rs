//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1093/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1093<F: Float>(t39951: F, t30511: F, t22934: F, t22939: F, t3443: F, t1803: F, t185: F, t186: F, t22968: F, t3399: F, t3445: F, t22982: F, t22986: F, t22988: F, t22994: F) -> (F, F, F, F, F, F, F, F) {
    let t47527 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t39951;
    let t47528 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t30511;
    let t47529 = F::cast_from(64.0_f64) / F::cast_from(405.0_f64) * t22934;
    let t47530 = F::cast_from(128.0_f64) / F::cast_from(405.0_f64) * t22939;
    let t47531 = t3443 * t3443;
    let t47535 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t185 * t186 * t1803 * t47531;
    let t47536 = F::cast_from(64.0_f64) / F::cast_from(405.0_f64) * t22968;
    let t47538 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3399 * t3445;
    let t47543 = t47527 - t47528 + t47529 + t47530 + t47535 - t47536 - t47538 + F::cast_from(0.60617527037037037035e-2_f64) * t22982 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t22986 - F::cast_from(0.5402469135802469136e-1_f64) * t22988 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t22994;
    (t47527, t47528, t47529, t47530, t47535, t47536, t47538, t47543)
}
